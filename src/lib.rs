//! Application logic library for Embassy nRF52 project
//!
//! This module contains the core application logic that can be tested
//! without requiring actual hardware.

#![cfg_attr(not(test), no_std)]

use core::fmt;

/// Blinky pattern state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlinkyState {
    On,
    Off,
}

impl BlinkyState {
    /// Toggle the state
    pub fn toggle(&mut self) {
        *self = match self {
            BlinkyState::On => BlinkyState::Off,
            BlinkyState::Off => BlinkyState::On,
        };
    }

    /// Get the next state
    pub fn next(&self) -> Self {
        match self {
            BlinkyState::On => BlinkyState::Off,
            BlinkyState::Off => BlinkyState::On,
        }
    }

    /// Convert to boolean (true = on, false = off)
    pub fn as_bool(&self) -> bool {
        matches!(self, BlinkyState::On)
    }
}

impl fmt::Display for BlinkyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlinkyState::On => write!(f, "ON"),
            BlinkyState::Off => write!(f, "OFF"),
        }
    }
}

/// Blinky pattern configuration
#[derive(Debug, Clone, Copy)]
pub struct BlinkyConfig {
    /// Duration in milliseconds for ON state
    pub on_duration_ms: u32,
    /// Duration in milliseconds for OFF state
    pub off_duration_ms: u32,
}

impl Default for BlinkyConfig {
    fn default() -> Self {
        Self {
            on_duration_ms: 500,
            off_duration_ms: 500,
        }
    }
}

impl BlinkyConfig {
    /// Create a new configuration
    pub const fn new(on_duration_ms: u32, off_duration_ms: u32) -> Self {
        Self {
            on_duration_ms,
            off_duration_ms,
        }
    }

    /// Create a fast blink pattern
    pub const fn fast() -> Self {
        Self::new(100, 100)
    }

    /// Create a slow blink pattern
    pub const fn slow() -> Self {
        Self::new(1000, 1000)
    }

    /// Get duration for current state
    pub fn duration_for_state(&self, state: BlinkyState) -> u32 {
        match state {
            BlinkyState::On => self.on_duration_ms,
            BlinkyState::Off => self.off_duration_ms,
        }
    }

    /// Validate configuration
    pub fn is_valid(&self) -> bool {
        self.on_duration_ms > 0 && self.off_duration_ms > 0
    }
}

/// A pattern generator for LED blinking
pub struct BlinkyPattern {
    state: BlinkyState,
    config: BlinkyConfig,
    pub(crate) cycle_count: u32,
}

impl BlinkyPattern {
    /// Create a new blinky pattern
    pub fn new(config: BlinkyConfig) -> Self {
        Self {
            state: BlinkyState::Off,
            config,
            cycle_count: 0,
        }
    }

    /// Get the current state
    pub fn state(&self) -> BlinkyState {
        self.state
    }

    /// Get the current cycle count
    pub fn cycle_count(&self) -> u32 {
        self.cycle_count
    }

    /// Get the configuration
    pub fn config(&self) -> BlinkyConfig {
        self.config
    }

    /// Advance to the next state and return the duration to wait
    pub fn next(&mut self) -> (BlinkyState, u32) {
        self.state.toggle();

        // Increment cycle count when transitioning to ON
        if self.state == BlinkyState::On {
            self.cycle_count = self.cycle_count.saturating_add(1);
        }

        let duration = self.config.duration_for_state(self.state);
        (self.state, duration)
    }

    /// Reset the pattern to initial state
    pub fn reset(&mut self) {
        self.state = BlinkyState::Off;
        self.cycle_count = 0;
    }

    /// Set cycle count for testing purposes (only available in test builds)
    #[doc(hidden)]
    pub fn set_cycle_count_for_test(&mut self, count: u32) {
        self.cycle_count = count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinky_state_toggle() {
        let mut state = BlinkyState::Off;
        assert_eq!(state, BlinkyState::Off);

        state.toggle();
        assert_eq!(state, BlinkyState::On);

        state.toggle();
        assert_eq!(state, BlinkyState::Off);
    }

    #[test]
    fn test_blinky_state_next() {
        let state = BlinkyState::Off;
        assert_eq!(state.next(), BlinkyState::On);

        let state = BlinkyState::On;
        assert_eq!(state.next(), BlinkyState::Off);
    }

    #[test]
    fn test_blinky_state_as_bool() {
        assert!(BlinkyState::On.as_bool());
        assert!(!BlinkyState::Off.as_bool());
    }

    #[test]
    fn test_blinky_config_default() {
        let config = BlinkyConfig::default();
        assert_eq!(config.on_duration_ms, 500);
        assert_eq!(config.off_duration_ms, 500);
        assert!(config.is_valid());
    }

    #[test]
    fn test_blinky_config_presets() {
        let fast = BlinkyConfig::fast();
        assert_eq!(fast.on_duration_ms, 100);
        assert_eq!(fast.off_duration_ms, 100);

        let slow = BlinkyConfig::slow();
        assert_eq!(slow.on_duration_ms, 1000);
        assert_eq!(slow.off_duration_ms, 1000);
    }

    #[test]
    fn test_blinky_config_duration_for_state() {
        let config = BlinkyConfig::new(200, 800);
        assert_eq!(config.duration_for_state(BlinkyState::On), 200);
        assert_eq!(config.duration_for_state(BlinkyState::Off), 800);
    }

    #[test]
    fn test_blinky_config_validation() {
        let valid = BlinkyConfig::new(100, 200);
        assert!(valid.is_valid());

        let invalid1 = BlinkyConfig::new(0, 200);
        assert!(!invalid1.is_valid());

        let invalid2 = BlinkyConfig::new(100, 0);
        assert!(!invalid2.is_valid());
    }

    #[test]
    fn test_blinky_pattern_initial_state() {
        let pattern = BlinkyPattern::new(BlinkyConfig::default());
        assert_eq!(pattern.state(), BlinkyState::Off);
        assert_eq!(pattern.cycle_count(), 0);
    }

    #[test]
    fn test_blinky_pattern_transitions() {
        let mut pattern = BlinkyPattern::new(BlinkyConfig::new(100, 200));

        // First transition: Off -> On
        let (state, duration) = pattern.next();
        assert_eq!(state, BlinkyState::On);
        assert_eq!(duration, 100);
        assert_eq!(pattern.cycle_count(), 1);

        // Second transition: On -> Off
        let (state, duration) = pattern.next();
        assert_eq!(state, BlinkyState::Off);
        assert_eq!(duration, 200);
        assert_eq!(pattern.cycle_count(), 1);

        // Third transition: Off -> On (new cycle)
        let (state, duration) = pattern.next();
        assert_eq!(state, BlinkyState::On);
        assert_eq!(duration, 100);
        assert_eq!(pattern.cycle_count(), 2);
    }

    #[test]
    fn test_blinky_pattern_cycle_counting() {
        let mut pattern = BlinkyPattern::new(BlinkyConfig::default());

        assert_eq!(pattern.cycle_count(), 0);

        // Complete 3 full cycles
        for i in 1..=3 {
            pattern.next(); // Off -> On
            assert_eq!(pattern.cycle_count(), i);
            pattern.next(); // On -> Off
            assert_eq!(pattern.cycle_count(), i);
        }
    }

    #[test]
    fn test_blinky_pattern_reset() {
        let mut pattern = BlinkyPattern::new(BlinkyConfig::default());

        // Advance a few times
        pattern.next();
        pattern.next();
        pattern.next();

        assert!(pattern.cycle_count() > 0);
        assert_eq!(pattern.state(), BlinkyState::On);

        // Reset
        pattern.reset();
        assert_eq!(pattern.cycle_count(), 0);
        assert_eq!(pattern.state(), BlinkyState::Off);
    }

    #[test]
    fn test_blinky_pattern_saturating_counter() {
        let mut pattern = BlinkyPattern::new(BlinkyConfig::default());

        // Set cycle count to near max
        pattern.cycle_count = u32::MAX - 1;

        // Transition to On (should increment)
        pattern.next();
        assert_eq!(pattern.cycle_count(), u32::MAX);

        // Transition to On again (should saturate, not overflow)
        pattern.next(); // Off
        pattern.next(); // On
        assert_eq!(pattern.cycle_count(), u32::MAX);
    }
}
