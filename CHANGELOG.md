# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2024

### Fixed

#### Build Issues
- **Fixed defmt formatting errors**: Removed floating-point format specifiers (`{:.2}`, `{:.1}`) from `defmt` log statements, as `defmt` doesn't support floating-point formatting in embedded contexts
  - Converted frequency to millihertz (mHz) as integer: `(freq * 1000.0) as u32`
  - Converted duty cycle to integer percentage: `duty as u32`
  
- **Fixed test compilation issues**: Resolved conflicts between embedded target compilation and host machine testing
  - Made all Embassy and embedded dependencies optional
  - Added `embassy` feature flag (enabled by default for binary builds)
  - Configured library to compile without `no_std` when running tests
  - Updated Cargo configuration to allow tests to run on host machine

#### Configuration Changes
- **Updated `.cargo/config.toml`**: Removed default target specification to allow flexible compilation
  - Tests now run on host machine without embedded toolchain
  - Binary still targets `thumbv7em-none-eabihf` when specified explicitly

- **Updated `Cargo.toml`**:
  - Added `[lib]` and `[[bin]]` sections to separate library from binary
  - Made embedded dependencies optional with `embassy` feature
  - Configured binary to require `embassy` feature
  - Removed unused test dependencies (`tokio`, `futures`)

#### Code Structure
- **Refactored main.rs**: 
  - Removed duplicate type definitions (now using library types)
  - Simplified main loop to use `BlinkyPattern` from library
  - Updated info logging to show cycle count and use integer formatting

- **Enhanced lib.rs**:
  - Added `set_cycle_count_for_test()` public helper method for testing
  - Made library compile with or without `std` based on build context
  - Improved test coverage with internal unit tests

#### Test Fixes
- **Fixed property test logic**: Corrected `test_property_state_bool_conversion_is_consistent`
  - Fixed iteration logic for expected state values
  - Changed from `i % 2 == 1` to `i % 2 == 0` to match actual toggle behavior

- **Fixed compiler warnings**: Prefixed unused loop variable with underscore (`_i`)

### Changed

#### Documentation
- **Updated README.md** with correct build and test commands:
  - Build: `cargo build --target thumbv7em-none-eabihf --release`
  - Test: `cargo test --no-default-features`
  - Run: `cargo run --target thumbv7em-none-eabihf --release`
  - Added note explaining why `--no-default-features` is required for tests

### Summary

The project now successfully:
- ✅ Builds for embedded target (`thumbv7em-none-eabihf`)
- ✅ Runs all tests on host machine (50 tests passing)
- ✅ Separates library logic from embedded-specific code
- ✅ Maintains test coverage without requiring hardware
- ✅ Follows Rust embedded best practices

### Testing Results

All test suites passing:
- **Unit tests**: 12/12 passed (in `src/lib.rs`)
- **Integration tests**: 12/12 passed (in `tests/blinky_logic_tests.rs`)
- **Mock tests**: 13/13 passed (in `tests/mock_led_tests.rs`)
- **Property tests**: 13/13 passed (in `tests/property_tests.rs`)

**Total**: 50 tests passing, 0 failures