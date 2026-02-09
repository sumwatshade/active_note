//! Integration tests for blinky logic
//!
//! These tests run on the host machine and verify the application logic
//! without requiring actual hardware.

use active_note::{BlinkyConfig, BlinkyPattern, BlinkyState};

#[test]
fn test_complete_blink_cycle() {
    let config = BlinkyConfig::new(250, 750);
    let mut pattern = BlinkyPattern::new(config);

    // Initial state should be Off
    assert_eq!(pattern.state(), BlinkyState::Off);
    assert_eq!(pattern.cycle_count(), 0);

    // First transition: Off -> On
    let (state, duration) = pattern.next();
    assert_eq!(state, BlinkyState::On);
    assert_eq!(duration, 250);
    assert_eq!(pattern.cycle_count(), 1);

    // Second transition: On -> Off
    let (state, duration) = pattern.next();
    assert_eq!(state, BlinkyState::Off);
    assert_eq!(duration, 750);
    assert_eq!(pattern.cycle_count(), 1);

    // Complete another full cycle
    let (state, _) = pattern.next();
    assert_eq!(state, BlinkyState::On);
    assert_eq!(pattern.cycle_count(), 2);
}

#[test]
fn test_multiple_blink_cycles() {
    let mut pattern = BlinkyPattern::new(BlinkyConfig::default());

    // Run 10 complete cycles
    for expected_cycle in 1..=10 {
        pattern.next(); // Off -> On
        assert_eq!(pattern.cycle_count(), expected_cycle);
        pattern.next(); // On -> Off
        assert_eq!(pattern.cycle_count(), expected_cycle);
    }
}

#[test]
fn test_fast_blink_pattern() {
    let config = BlinkyConfig::fast();
    let mut pattern = BlinkyPattern::new(config);

    // Fast blink should have 100ms intervals
    let (_, on_duration) = pattern.next();
    assert_eq!(on_duration, 100);

    let (_, off_duration) = pattern.next();
    assert_eq!(off_duration, 100);
}

#[test]
fn test_slow_blink_pattern() {
    let config = BlinkyConfig::slow();
    let mut pattern = BlinkyPattern::new(config);

    // Slow blink should have 1000ms intervals
    let (_, on_duration) = pattern.next();
    assert_eq!(on_duration, 1000);

    let (_, off_duration) = pattern.next();
    assert_eq!(off_duration, 1000);
}

#[test]
fn test_asymmetric_blink_pattern() {
    // Short ON, long OFF (like a heartbeat)
    let config = BlinkyConfig::new(100, 900);
    let mut pattern = BlinkyPattern::new(config);

    let (state1, duration1) = pattern.next();
    assert_eq!(state1, BlinkyState::On);
    assert_eq!(duration1, 100);

    let (state2, duration2) = pattern.next();
    assert_eq!(state2, BlinkyState::Off);
    assert_eq!(duration2, 900);
}

#[test]
fn test_pattern_reset_functionality() {
    let mut pattern = BlinkyPattern::new(BlinkyConfig::default());

    // Advance through several transitions
    for _ in 0..6 {
        pattern.next();
    }

    let cycles_before_reset = pattern.cycle_count();
    assert!(cycles_before_reset >= 3);

    // Reset should clear everything
    pattern.reset();
    assert_eq!(pattern.cycle_count(), 0);
    assert_eq!(pattern.state(), BlinkyState::Off);

    // After reset, should start fresh
    let (state, _) = pattern.next();
    assert_eq!(state, BlinkyState::On);
    assert_eq!(pattern.cycle_count(), 1);
}

#[test]
fn test_config_validation() {
    let valid_configs = vec![
        BlinkyConfig::new(1, 1),
        BlinkyConfig::new(100, 500),
        BlinkyConfig::new(u32::MAX, u32::MAX),
    ];

    for config in valid_configs {
        assert!(config.is_valid(), "Config should be valid: {:?}", config);
    }

    let invalid_configs = vec![
        BlinkyConfig::new(0, 100),
        BlinkyConfig::new(100, 0),
        BlinkyConfig::new(0, 0),
    ];

    for config in invalid_configs {
        assert!(!config.is_valid(), "Config should be invalid: {:?}", config);
    }
}

#[test]
fn test_state_display() {
    let on_str = format!("{}", BlinkyState::On);
    let off_str = format!("{}", BlinkyState::Off);

    assert_eq!(on_str, "ON");
    assert_eq!(off_str, "OFF");
}

#[test]
fn test_state_conversion_to_bool() {
    assert!(BlinkyState::On.as_bool());
    assert!(!BlinkyState::Off.as_bool());
}

#[test]
fn test_long_running_pattern() {
    // Test that pattern works correctly over many cycles
    let mut pattern = BlinkyPattern::new(BlinkyConfig::default());
    let mut on_count = 0;
    let mut off_count = 0;

    // Run 1000 transitions
    for _ in 0..1000 {
        let (state, _duration) = pattern.next();
        match state {
            BlinkyState::On => on_count += 1,
            BlinkyState::Off => off_count += 1,
        }
    }

    // Should have equal number of ON and OFF states
    assert_eq!(on_count, 500);
    assert_eq!(off_count, 500);
    assert_eq!(pattern.cycle_count(), 500);
}

#[test]
fn test_pattern_determinism() {
    // Two patterns with same config should behave identically
    let config = BlinkyConfig::new(123, 456);
    let mut pattern1 = BlinkyPattern::new(config);
    let mut pattern2 = BlinkyPattern::new(config);

    for _ in 0..20 {
        let (state1, duration1) = pattern1.next();
        let (state2, duration2) = pattern2.next();

        assert_eq!(state1, state2);
        assert_eq!(duration1, duration2);
        assert_eq!(pattern1.cycle_count(), pattern2.cycle_count());
    }
}

#[test]
fn test_edge_case_minimal_durations() {
    // Test with minimal valid durations
    let config = BlinkyConfig::new(1, 1);
    let mut pattern = BlinkyPattern::new(config);

    assert!(config.is_valid());

    let (state, duration) = pattern.next();
    assert_eq!(state, BlinkyState::On);
    assert_eq!(duration, 1);

    let (state, duration) = pattern.next();
    assert_eq!(state, BlinkyState::Off);
    assert_eq!(duration, 1);
}
