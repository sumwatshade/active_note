# Testing Guide

This document describes how to test the Embassy nRF52 firmware without actual hardware.

## Overview

The project includes multiple layers of testing:

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test component interactions
3. **Property Tests** - Verify invariants and properties
4. **Mock-based Tests** - Simulate hardware behavior

## Running Tests

### Run All Tests

```bash
cargo test --no-default-features
```

**Note:** The `--no-default-features` flag is required to run tests on the host machine without compiling embedded dependencies.

### Run Specific Test Suites

```bash
# Run only unit tests (in src/)
cargo test --lib --no-default-features

# Run only integration tests (in tests/)
cargo test --test blinky_logic_tests --no-default-features
cargo test --test mock_led_tests --no-default-features
cargo test --test property_tests --no-default-features
```

### Run Tests with Output

```bash
# Show println! output from tests
cargo test --no-default-features -- --nocapture

# Show test names as they run
cargo test --no-default-features -- --test-threads=1 --nocapture
```

### Run Specific Tests

```bash
# Run tests matching a pattern
cargo test --no-default-features blink

# Run a specific test
cargo test --no-default-features test_complete_blink_cycle
```

## Test Organization

### Unit Tests (`src/lib.rs`, `src/main.rs`)

Located in `#[cfg(test)] mod tests` blocks within source files. These test individual functions and basic logic:

- State transitions
- Configuration validation
- Duration calculations
- Duty cycle calculations

**Example:**
```rust
#[test]
fn test_led_state_transitions() {
    assert_eq!(LedState::Off.next(), LedState::On);
    assert_eq!(LedState::On.next(), LedState::Off);
}
```

### Integration Tests (`tests/`)

#### `blinky_logic_tests.rs`

Tests the complete blinky pattern logic including:
- Complete blink cycles
- Multiple patterns (fast, slow, asymmetric)
- Reset functionality
- Edge cases

#### `mock_led_tests.rs`

Uses mock implementations to simulate hardware:
- Mock LED that records state changes
- Mock timing that records delays
- Full blinky simulation with statistics

**Example:**
```rust
let mut led = MockLed::new();
led.set_high();
led.set_low();
assert_eq!(led.get_states(), vec![true, false]);
```

#### `property_tests.rs`

Tests properties that should always hold true:
- State toggles always alternate
- Cycle count never decreases
- Durations are consistent
- Reset returns to initial state

## Test Coverage

### What Is Tested

✅ **Business Logic**
- State machine transitions
- Configuration validation
- Timing calculations
- Cycle counting

✅ **Edge Cases**
- Zero durations (invalid)
- Maximum values (u32::MAX)
- Overflow protection (saturation)
- Reset behavior

✅ **Invariants**
- State alternation
- Non-decreasing counters
- Deterministic behavior
- Configuration consistency

### What Is NOT Tested (Requires Hardware)

❌ **Hardware-Specific**
- Actual GPIO pin manipulation
- Real-time timing accuracy
- Power consumption
- Physical LED behavior
- Interrupt handling

## Writing New Tests

### Adding a Unit Test

Add to the `#[cfg(test)]` module in your source file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_new_feature() {
        // Your test here
        assert_eq!(1 + 1, 2);
    }
}
```

### Adding an Integration Test

Create a new file in `tests/`:

```rust
// tests/my_new_test.rs
use active_note::{BlinkyPattern, BlinkyConfig};

#[test]
fn test_my_integration() {
    let pattern = BlinkyPattern::new(BlinkyConfig::default());
    // Your test here
}
```

### Creating a Mock

```rust
struct MockComponent {
    state: Vec<Action>,
}

impl MockComponent {
    fn new() -> Self {
        Self { state: Vec::new() }
    }

    fn record_action(&mut self, action: Action) {
        self.state.push(action);
    }

    fn verify(&self) -> bool {
        // Verification logic
        true
    }
}
```

## Continuous Integration

Tests are designed to run in CI/CD environments without hardware:

```yaml
# Example GitHub Actions
- name: Run tests
  run: cargo test --no-default-features
```

## Test Performance

Tests run on the host machine and complete quickly:

```bash
$ cargo test --no-default-features
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running unittests src/lib.rs (target/debug/deps/active_note-...)
     Running tests/blinky_logic_tests.rs (target/debug/deps/blinky_logic_tests-...)
     Running tests/mock_led_tests.rs (target/debug/deps/mock_led_tests-...)
     Running tests/property_tests.rs (target/debug/deps/property_tests-...)

test result: ok. 50 tests passed; 0 failed; 0 ignored; 0 measured
```

## Debugging Tests

### Run with Backtrace

```bash
RUST_BACKTRACE=1 cargo test --no-default-features
```

### Run a Single Failing Test

```bash
cargo test --no-default-features test_name -- --exact --nocapture
```

### Use Debug Printing

```rust
#[test]
fn test_debug() {
    let value = compute_something();
    println!("Debug value: {:?}", value);
    assert_eq!(value, expected);
}
```

Run with: `cargo test --no-default-features test_debug -- --nocapture`

## Best Practices

### Do's

✅ Test business logic separately from hardware
✅ Use descriptive test names
✅ Test edge cases and error conditions
✅ Keep tests fast and deterministic
✅ Use property-based testing for invariants

### Don'ts

❌ Don't test hardware-specific behavior without hardware
❌ Don't use `sleep()` or real delays in tests
❌ Don't rely on test execution order
❌ Don't test implementation details
❌ Don't write flaky tests

## Example Test Session

```bash
$ cargo test --no-default-features --test blinky_logic_tests

running 12 tests
test test_asymmetric_blink_pattern ... ok
test test_complete_blink_cycle ... ok
test test_config_validation ... ok
test test_edge_case_minimal_durations ... ok
test test_fast_blink_pattern ... ok
test test_long_running_pattern ... ok
test test_multiple_blink_cycles ... ok
test test_pattern_determinism ... ok
test test_pattern_reset_functionality ... ok
test test_slow_blink_pattern ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Further Reading

- [The Rust Programming Language - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust By Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Embassy Testing Strategies](https://embassy.dev/)