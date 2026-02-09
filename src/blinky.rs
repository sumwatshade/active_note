//! Blinky business logic module
//!
//! This module separates the blinking logic from hardware dependencies,
//! making it testable without actual hardware.

use core::future::Future;
use core::pin::Pin;

/// Trait for controlling an LED
/// This abstraction allows us to test without real hardware
pub trait Led {
    /// Turn the LED on
    fn set_high(&mut self);

    /// Turn the LED off
    fn set_low(&mut self);

    /// Toggle the LED state
    fn toggle(&mut self);
}

/// Trait for async delays
/// This abstraction allows us to test timing logic without real delays
pub trait AsyncDelay {
    /// Delay for the specified number of milliseconds
    fn delay_ms(&mut self, ms: u64) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

/// Blinky pattern configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlinkyConfig {
    /// Duration LED is on (milliseconds)
    pub on_duration_ms: u64,
    /// Duration LED is off (milliseconds)
    pub off_duration_ms: u64,
    /// Number of blinks (None = infinite)
    pub count: Option<u32>,
}

impl Default for BlinkyConfig {
    fn default() -> Self {
        Self {
            on_duration_ms: 500,
            off_duration_ms: 500,
            count: None, // Infinite by default
        }
    }
}

impl BlinkyConfig {
    /// Create a new blinky configuration
    pub fn new(on_duration_ms: u64, off_duration_ms: u64) -> Self {
        Self {
            on_duration_ms,
            off_duration_ms,
            count: None,
        }
    }

    /// Set the number of blinks
    pub fn with_count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Calculate the total period of one blink cycle
    pub fn period_ms(&self) -> u64 {
        self.on_duration_ms + self.off_duration_ms
    }

    /// Calculate frequency in Hz (rounded)
    pub fn frequency_hz(&self) -> f32 {
        1000.0 / self.period_ms() as f32
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.on_duration_ms == 0 && self.off_duration_ms == 0 {
            return Err("Both on and off durations cannot be zero");
        }
        if let Some(count) = self.count {
            if count == 0 {
                return Err("Blink count cannot be zero");
            }
        }
        Ok(())
    }
}

/// State machine for the blinky pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkyState {
    On,
    Off,
}

/// Blinky controller that manages the blinking pattern
pub struct BlinkyController {
    config: BlinkyConfig,
    state: BlinkyState,
    blink_count: u32,
}

impl BlinkyController {
    /// Create a new blinky controller with the given configuration
    pub fn new(config: BlinkyConfig) -> Result<Self, &'static str> {
        config.validate()?;
        Ok(Self {
            config,
            state: BlinkyState::Off,
            blink_count: 0,
        })
    }

    /// Get the current state
    pub fn state(&self) -> BlinkyState {
        self.state
    }

    /// Get the current blink count
    pub fn blink_count(&self) -> u32 {
        self.blink_count
    }

    /// Get the configuration
    pub fn config(&self) -> &BlinkyConfig {
        &self.config
    }

    /// Check if blinking should continue
    pub fn should_continue(&self) -> bool {
        match self.config.count {
            Some(max) => self.blink_count < max,
            None => true,
        }
    }

    /// Perform one blink cycle step
    /// Returns the duration to wait before the next step
    pub fn step<L: Led>(&mut self, led: &mut L) -> Option<u64> {
        if !self.should_continue() {
            return None;
        }

        match self.state {
            BlinkyState::Off => {
                led.set_high();
                self.state = BlinkyState::On;
                Some(self.config.on_duration_ms)
            }
            BlinkyState::On => {
                led.set_low();
                self.state = BlinkyState::Off;
                self.blink_count += 1;

                if self.should_continue() {
                    Some(self.config.off_duration_ms)
                } else {
                    None
                }
            }
        }
    }

    /// Run the complete blink pattern (async version)
    pub async fn run_async<L, D>(&mut self, led: &mut L, delay: &mut D)
    where
        L: Led,
        D: AsyncDelay,
    {
        while let Some(duration) = self.step(led) {
            delay.delay_ms(duration).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinky_config_default() {
        let config = BlinkyConfig::default();
        assert_eq!(config.on_duration_ms, 500);
        assert_eq!(config.off_duration_ms, 500);
        assert_eq!(config.count, None);
    }

    #[test]
    fn test_blinky_config_period() {
        let config = BlinkyConfig::new(300, 700);
        assert_eq!(config.period_ms(), 1000);
        assert_eq!(config.frequency_hz(), 1.0);
    }

    #[test]
    fn test_blinky_config_frequency() {
        let config = BlinkyConfig::new(500, 500);
        assert_eq!(config.frequency_hz(), 1.0);

        let config = BlinkyConfig::new(250, 250);
        assert_eq!(config.frequency_hz(), 2.0);
    }

    #[test]
    fn test_blinky_config_validation() {
        let config = BlinkyConfig::new(100, 100);
        assert!(config.validate().is_ok());

        let config = BlinkyConfig::new(0, 0);
        assert!(config.validate().is_err());

        let config = BlinkyConfig::new(100, 100).with_count(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_blinky_controller_creation() {
        let config = BlinkyConfig::default();
        let controller = BlinkyController::new(config);
        assert!(controller.is_ok());

        let controller = controller.unwrap();
        assert_eq!(controller.state(), BlinkyState::Off);
        assert_eq!(controller.blink_count(), 0);
    }

    #[test]
    fn test_blinky_controller_should_continue() {
        let config = BlinkyConfig::default().with_count(3);
        let mut controller = BlinkyController::new(config).unwrap();

        assert!(controller.should_continue());
        controller.blink_count = 2;
        assert!(controller.should_continue());
        controller.blink_count = 3;
        assert!(!controller.should_continue());
    }

    #[test]
    fn test_blinky_controller_infinite() {
        let config = BlinkyConfig::default();
        let controller = BlinkyController::new(config).unwrap();

        // Should always continue with infinite count
        assert!(controller.should_continue());
    }
}
