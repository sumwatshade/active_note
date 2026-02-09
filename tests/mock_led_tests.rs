//! Mock-based tests for LED behavior
//!
//! These tests use mock implementations to verify the blinky logic
//! without requiring actual hardware.

use std::cell::RefCell;
use std::rc::Rc;

/// Mock LED implementation that records state changes
#[derive(Clone)]
struct MockLed {
    states: Rc<RefCell<Vec<bool>>>,
}

impl MockLed {
    fn new() -> Self {
        Self {
            states: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn set_high(&mut self) {
        self.states.borrow_mut().push(true);
    }

    fn set_low(&mut self) {
        self.states.borrow_mut().push(false);
    }

    fn get_states(&self) -> Vec<bool> {
        self.states.borrow().clone()
    }

    fn state_count(&self) -> usize {
        self.states.borrow().len()
    }

    fn last_state(&self) -> Option<bool> {
        self.states.borrow().last().copied()
    }

    fn clear(&mut self) {
        self.states.borrow_mut().clear();
    }
}

#[test]
fn test_mock_led_records_states() {
    let mut led = MockLed::new();

    led.set_high();
    led.set_low();
    led.set_high();

    let states = led.get_states();
    assert_eq!(states, vec![true, false, true]);
}

#[test]
fn test_mock_led_state_count() {
    let mut led = MockLed::new();

    assert_eq!(led.state_count(), 0);

    led.set_high();
    assert_eq!(led.state_count(), 1);

    led.set_low();
    assert_eq!(led.state_count(), 2);
}

#[test]
fn test_mock_led_last_state() {
    let mut led = MockLed::new();

    assert_eq!(led.last_state(), None);

    led.set_high();
    assert_eq!(led.last_state(), Some(true));

    led.set_low();
    assert_eq!(led.last_state(), Some(false));
}

#[test]
fn test_mock_led_clear() {
    let mut led = MockLed::new();

    led.set_high();
    led.set_low();
    assert_eq!(led.state_count(), 2);

    led.clear();
    assert_eq!(led.state_count(), 0);
    assert_eq!(led.last_state(), None);
}

#[test]
fn test_blink_pattern_alternates() {
    let mut led = MockLed::new();

    // Simulate a simple blink pattern
    for _ in 0..5 {
        led.set_high();
        led.set_low();
    }

    let states = led.get_states();
    assert_eq!(states.len(), 10);

    // Verify alternating pattern
    for (i, &state) in states.iter().enumerate() {
        if i % 2 == 0 {
            assert!(state, "Even indices should be HIGH");
        } else {
            assert!(!state, "Odd indices should be LOW");
        }
    }
}

#[test]
fn test_asymmetric_blink_pattern() {
    let mut led = MockLed::new();

    // Simulate 3 short blinks followed by a pause
    for _ in 0..3 {
        led.set_high();
        led.set_low();
    }

    assert_eq!(led.state_count(), 6);
    assert_eq!(led.last_state(), Some(false));

    // Verify we have 3 HIGH states and 3 LOW states
    let states = led.get_states();
    let high_count = states.iter().filter(|&&s| s).count();
    let low_count = states.iter().filter(|&&s| !s).count();

    assert_eq!(high_count, 3);
    assert_eq!(low_count, 3);
}

/// Mock timing recorder
struct MockTiming {
    delays: Vec<u64>,
}

impl MockTiming {
    fn new() -> Self {
        Self { delays: Vec::new() }
    }

    fn delay_ms(&mut self, ms: u64) {
        self.delays.push(ms);
    }

    fn total_time(&self) -> u64 {
        self.delays.iter().sum()
    }

    fn delay_count(&self) -> usize {
        self.delays.len()
    }

    fn get_delays(&self) -> &[u64] {
        &self.delays
    }
}

#[test]
fn test_mock_timing_records_delays() {
    let mut timing = MockTiming::new();

    timing.delay_ms(100);
    timing.delay_ms(200);
    timing.delay_ms(300);

    assert_eq!(timing.delay_count(), 3);
    assert_eq!(timing.total_time(), 600);
    assert_eq!(timing.get_delays(), &[100, 200, 300]);
}

#[test]
fn test_blink_with_timing() {
    let mut led = MockLed::new();
    let mut timing = MockTiming::new();

    // Simulate blink pattern with timing
    let on_duration = 100;
    let off_duration = 200;

    for _ in 0..3 {
        led.set_high();
        timing.delay_ms(on_duration);

        led.set_low();
        timing.delay_ms(off_duration);
    }

    // Verify LED states
    assert_eq!(led.state_count(), 6);

    // Verify timing
    assert_eq!(timing.delay_count(), 6);
    assert_eq!(timing.total_time(), 3 * (on_duration + off_duration));

    // Verify alternating delays
    let delays = timing.get_delays();
    for (i, &delay) in delays.iter().enumerate() {
        if i % 2 == 0 {
            assert_eq!(delay, on_duration);
        } else {
            assert_eq!(delay, off_duration);
        }
    }
}

#[test]
fn test_duty_cycle_calculation_from_mock() {
    let mut timing = MockTiming::new();

    let on_duration = 750;
    let off_duration = 250;

    for _ in 0..10 {
        timing.delay_ms(on_duration);
        timing.delay_ms(off_duration);
    }

    let total = timing.total_time();
    let on_time: u64 = timing
        .get_delays()
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, &d)| d)
        .sum();

    let duty_cycle = (on_time as f64 / total as f64) * 100.0;

    assert!((duty_cycle - 75.0).abs() < 0.1);
}

/// Combined mock for full blinky simulation
struct BlinkySimulator {
    led: MockLed,
    timing: MockTiming,
}

impl BlinkySimulator {
    fn new() -> Self {
        Self {
            led: MockLed::new(),
            timing: MockTiming::new(),
        }
    }

    fn blink_once(&mut self, on_ms: u64, off_ms: u64) {
        self.led.set_high();
        self.timing.delay_ms(on_ms);
        self.led.set_low();
        self.timing.delay_ms(off_ms);
    }

    fn blink_n_times(&mut self, count: usize, on_ms: u64, off_ms: u64) {
        for _ in 0..count {
            self.blink_once(on_ms, off_ms);
        }
    }

    fn verify_pattern(&self) -> bool {
        let states = self.led.get_states();

        // Check that we have an even number of states
        if states.len() % 2 != 0 {
            return false;
        }

        // Check alternating pattern
        for (i, &state) in states.iter().enumerate() {
            let expected = i % 2 == 0;
            if state != expected {
                return false;
            }
        }

        true
    }

    fn get_statistics(&self) -> BlinkyStats {
        let states = self.led.get_states();
        let high_count = states.iter().filter(|&&s| s).count();
        let low_count = states.iter().filter(|&&s| !s).count();
        let total_time = self.timing.total_time();

        BlinkyStats {
            high_count,
            low_count,
            total_time_ms: total_time,
            state_changes: states.len(),
        }
    }
}

struct BlinkyStats {
    high_count: usize,
    low_count: usize,
    total_time_ms: u64,
    state_changes: usize,
}

#[test]
fn test_blinky_simulator_single_blink() {
    let mut sim = BlinkySimulator::new();
    sim.blink_once(500, 500);

    assert!(sim.verify_pattern());

    let stats = sim.get_statistics();
    assert_eq!(stats.high_count, 1);
    assert_eq!(stats.low_count, 1);
    assert_eq!(stats.total_time_ms, 1000);
    assert_eq!(stats.state_changes, 2);
}

#[test]
fn test_blinky_simulator_multiple_blinks() {
    let mut sim = BlinkySimulator::new();
    sim.blink_n_times(5, 100, 200);

    assert!(sim.verify_pattern());

    let stats = sim.get_statistics();
    assert_eq!(stats.high_count, 5);
    assert_eq!(stats.low_count, 5);
    assert_eq!(stats.total_time_ms, 1500); // 5 * (100 + 200)
    assert_eq!(stats.state_changes, 10);
}

#[test]
fn test_blinky_simulator_fast_pattern() {
    let mut sim = BlinkySimulator::new();
    sim.blink_n_times(10, 50, 50);

    assert!(sim.verify_pattern());

    let stats = sim.get_statistics();
    assert_eq!(stats.high_count, 10);
    assert_eq!(stats.low_count, 10);
    assert_eq!(stats.total_time_ms, 1000);
}

#[test]
fn test_blinky_simulator_slow_pattern() {
    let mut sim = BlinkySimulator::new();
    sim.blink_n_times(2, 2000, 2000);

    assert!(sim.verify_pattern());

    let stats = sim.get_statistics();
    assert_eq!(stats.high_count, 2);
    assert_eq!(stats.low_count, 2);
    assert_eq!(stats.total_time_ms, 8000);
}
