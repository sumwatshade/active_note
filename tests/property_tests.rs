//! Property-based tests for the firmware logic
//!
//! These tests verify properties that should always hold true,
//! regardless of specific input values.

use active_note::{BlinkyConfig, BlinkyPattern, BlinkyState};

#[test]
fn test_property_state_toggles_alternate() {
    // Property: Toggling state should alternate between On and Off
    let mut state = BlinkyState::Off;
    for _ in 0..100 {
        let prev = state;
        state.toggle();
        assert_ne!(state, prev, "State should change on toggle");
        state.toggle();
        assert_eq!(
            state, prev,
            "State should return to original after two toggles"
        );
    }
}

#[test]
fn test_property_cycle_count_never_decreases() {
    // Property: Cycle count should never decrease (except on reset)
    let config = BlinkyConfig::default();
    let mut pattern = BlinkyPattern::new(config);

    let mut prev_count = pattern.cycle_count();

    for _ in 0..100 {
        pattern.next();
        let current_count = pattern.cycle_count();
        assert!(
            current_count >= prev_count,
            "Cycle count should never decrease without reset"
        );
        prev_count = current_count;
    }
}

#[test]
fn test_property_on_duration_always_consistent() {
    // Property: When in ON state, the duration should match config
    let config = BlinkyConfig::new(123, 456);
    let mut pattern = BlinkyPattern::new(config);

    for _ in 0..50 {
        let (state, duration) = pattern.next();
        if state == BlinkyState::On {
            assert_eq!(duration, 123, "ON duration should match config");
        }
        pattern.next(); // Advance to next state
    }
}

#[test]
fn test_property_off_duration_always_consistent() {
    // Property: When in OFF state, the duration should match config
    let config = BlinkyConfig::new(123, 456);
    let mut pattern = BlinkyPattern::new(config);

    pattern.next(); // Move to ON first

    for _ in 0..50 {
        pattern.next(); // Move to OFF
        let (state, duration) = pattern.next();
        if state == BlinkyState::Off {
            assert_eq!(duration, 456, "OFF duration should match config");
        }
    }
}

#[test]
fn test_property_cycle_increments_on_on_transition() {
    // Property: Cycle count increases only when transitioning to ON
    let config = BlinkyConfig::default();
    let mut pattern = BlinkyPattern::new(config);

    for _i in 1..=20 {
        let prev_count = pattern.cycle_count();
        let (state, _) = pattern.next();

        if state == BlinkyState::On {
            assert_eq!(
                pattern.cycle_count(),
                prev_count + 1,
                "Cycle should increment on ON transition"
            );
        } else {
            assert_eq!(
                pattern.cycle_count(),
                prev_count,
                "Cycle should not increment on OFF transition"
            );
        }
    }
}

#[test]
fn test_property_reset_returns_to_initial_state() {
    // Property: Reset always returns pattern to initial state
    let configs = [
        BlinkyConfig::default(),
        BlinkyConfig::fast(),
        BlinkyConfig::slow(),
        BlinkyConfig::new(73, 197),
    ];

    for config in configs {
        let mut pattern = BlinkyPattern::new(config);

        // Advance to various states
        for _ in 0..10 {
            pattern.next();
        }

        // Reset
        pattern.reset();

        // Verify initial state
        assert_eq!(pattern.state(), BlinkyState::Off);
        assert_eq!(pattern.cycle_count(), 0);
    }
}

#[test]
fn test_property_state_sequence_is_deterministic() {
    // Property: Given the same starting state, the sequence should be identical
    let config = BlinkyConfig::new(100, 200);

    let mut pattern1 = BlinkyPattern::new(config);
    let mut pattern2 = BlinkyPattern::new(config);

    for _ in 0..50 {
        let (state1, dur1) = pattern1.next();
        let (state2, dur2) = pattern2.next();

        assert_eq!(state1, state2, "States should match");
        assert_eq!(dur1, dur2, "Durations should match");
    }
}

#[test]
fn test_property_valid_configs_are_always_usable() {
    // Property: Any valid config should work without panics
    let test_configs = [
        BlinkyConfig::new(1, 1),
        BlinkyConfig::new(1, u32::MAX),
        BlinkyConfig::new(u32::MAX, 1),
        BlinkyConfig::new(12345, 67890),
    ];

    for config in test_configs {
        assert!(config.is_valid(), "Config should be valid");

        let mut pattern = BlinkyPattern::new(config);

        // Should not panic
        for _ in 0..10 {
            pattern.next();
        }
    }
}

#[test]
fn test_property_state_bool_conversion_is_consistent() {
    // Property: Bool conversion should be consistent with state
    assert_eq!(BlinkyState::On.as_bool(), true);
    assert_eq!(BlinkyState::Off.as_bool(), false);

    // And it should remain consistent through toggles
    let mut state = BlinkyState::Off;
    for i in 0..100 {
        state.toggle();
        let expected = i % 2 == 0; // Even iterations should be On (0, 2, 4...)
        assert_eq!(state.as_bool(), expected);
    }
}

#[test]
fn test_property_next_and_toggle_are_equivalent() {
    // Property: state.next() should equal the result of toggling
    let test_states = [BlinkyState::On, BlinkyState::Off];

    for initial_state in test_states {
        let next_state = initial_state.next();

        let mut toggled_state = initial_state;
        toggled_state.toggle();

        assert_eq!(
            next_state, toggled_state,
            "next() and toggle() should be equivalent"
        );
    }
}

#[test]
fn test_property_config_durations_match_usage() {
    // Property: The duration returned should always match what's in the config
    let test_cases = [(100, 200), (1, 999), (500, 500), (1000, 1)];

    for (on_ms, off_ms) in test_cases {
        let config = BlinkyConfig::new(on_ms, off_ms);

        assert_eq!(config.duration_for_state(BlinkyState::On), on_ms);
        assert_eq!(config.duration_for_state(BlinkyState::Off), off_ms);
    }
}

#[test]
fn test_property_invalid_configs_are_detected() {
    // Property: Invalid configs should be detected
    let invalid_configs = [
        BlinkyConfig::new(0, 100),
        BlinkyConfig::new(100, 0),
        BlinkyConfig::new(0, 0),
    ];

    for config in invalid_configs {
        assert!(
            !config.is_valid(),
            "Config with zero duration should be invalid"
        );
    }
}

#[test]
fn test_property_cycle_count_saturates() {
    // Property: Cycle count should saturate at u32::MAX, not overflow
    let mut pattern = BlinkyPattern::new(BlinkyConfig::default());

    // Manually set to near max
    pattern.set_cycle_count_for_test(u32::MAX - 1);

    pattern.next(); // Should increment to MAX
    assert_eq!(pattern.cycle_count(), u32::MAX);

    // Further increments should stay at MAX
    pattern.next(); // OFF
    pattern.next(); // ON - would overflow if not saturating
    assert_eq!(pattern.cycle_count(), u32::MAX);
}
