# Embassy nRF52 Project

An embedded Rust project using [Embassy](https://embassy.dev/) async framework with the nRF52 HAL.

## Overview

This project provides a scaffold for building async embedded applications targeting Nordic Semiconductor nRF52 microcontrollers using the Embassy framework.

## Features

- **Async/await** support for embedded development
- **Embassy executor** for cooperative multitasking
- **embassy-nrf HAL** for nRF52 peripheral access
- **defmt** for efficient logging
- **probe-rs** for flashing and debugging

## Requirements

- Rust toolchain (install via [rustup](https://rustup.rs/))
- [probe-rs](https://probe.rs/) for flashing and debugging
- An nRF52 development board (e.g., nRF52840-DK, nRF52833-DK)
- A debug probe (J-Link, CMSIS-DAP, or the on-board debugger)

## Getting Started

### Install Dependencies

```bash
# Install probe-rs
cargo install probe-rs-tools

# The correct target will be installed automatically via rust-toolchain.toml
```

### Build

To build the firmware for the embedded target:

```bash
cargo build --target thumbv7em-none-eabihf --release
```

### Flash and Run

Connect your nRF52 board via a debug probe, then:

```bash
cargo run --target thumbv7em-none-eabihf --release
```

## Project Structure

```
.
├── .cargo/
│   └── config.toml     # Cargo configuration (runner, target)
├── src/
│   └── main.rs         # Main application entry point
├── build.rs            # Build script for linker configuration
├── Cargo.toml          # Project dependencies and configuration
├── memory.x            # Memory layout definition (optional, auto-generated)
└── rust-toolchain.toml # Rust toolchain specification
```

## Testing

This project includes comprehensive tests that can be run **without hardware**. The tests are organized into three categories:

### Unit Tests

Unit tests are embedded in the source files and test individual functions and logic:

```bash
cargo test --lib --no-default-features
```

### Integration Tests

Integration tests verify the application logic with mock implementations:

```bash
cargo test --test blinky_logic_tests --no-default-features
cargo test --test mock_led_tests --no-default-features
cargo test --test property_tests --no-default-features
```

### Run All Tests

To run all tests at once:

```bash
cargo test --no-default-features
```

**Note:** Tests must be run with `--no-default-features` to avoid compiling embedded dependencies on the host machine.

### Test Coverage

The test suite includes:

- **Unit tests** in `src/lib.rs` and `src/main.rs` - Pure logic tests
- **Mock-based tests** in `tests/mock_led_tests.rs` - Hardware simulation
- **Property tests** in `tests/property_tests.rs` - Invariant verification
- **Integration tests** in `tests/blinky_logic_tests.rs` - End-to-end logic

### What's Being Tested

- ✅ LED state transitions and patterns
- ✅ Timing calculations and duty cycles
- ✅ Configuration validation
- ✅ State machine behavior
- ✅ Edge cases and error conditions
- ✅ Long-running pattern stability
- ✅ Deterministic behavior

All tests run on your host machine using standard Rust tooling - no hardware or embedded toolchain required!

## Configuration

### Changing the Target Chip

By default, this project targets the **nRF52840**. To use a different chip:

1. Update the chip feature in `Cargo.toml`:
   ```toml
   embassy-nrf = { version = "0.3", features = ["nrf52833", ...] }
   ```

2. Update the chip in `.cargo/config.toml`:
   ```toml
   runner = "probe-rs run --chip nRF52833_xxAA"
   ```

### Available nRF52 Chips

- `nrf52805`
- `nrf52810`
- `nrf52811`
- `nrf52820`
- `nrf52832`
- `nrf52833`
- `nrf52840`

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Development Workflow

### Typical Development Cycle

1. **Write/modify logic** in `src/lib.rs` or `src/main.rs`
2. **Run tests** with `cargo test --no-default-features` to verify logic
3. **Build firmware** with `cargo build --target thumbv7em-none-eabihf --release`
4. **Flash to hardware** with `cargo run --target thumbv7em-none-eabihf --release`
5. **Debug** using defmt logs over RTT

### Testing Strategy

The project separates hardware-dependent code from business logic:

- **Business logic** (in `src/lib.rs`) - Fully testable without hardware
- **Hardware interface** (in `src/main.rs`) - Minimal glue code

This allows you to:
- Develop and test logic rapidly on your host machine
- Catch bugs before flashing to hardware
- Use standard Rust testing and debugging tools
- Maintain confidence in your firmware through CI/CD