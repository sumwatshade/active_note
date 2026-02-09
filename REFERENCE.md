# Active Note - Quick Reference Guide

This document provides a quick reference for common commands and workflows.

## Prerequisites

```bash
# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install probe-rs tools for flashing/debugging
cargo install probe-rs-tools
```

The embedded target (`thumbv7em-none-eabihf`) will be installed automatically via `rust-toolchain.toml`.

---

## Testing (Host Machine)

All tests run on your development machine without requiring hardware.

### Run All Tests
```bash
cargo test --no-default-features
```

### Run Specific Test Suites
```bash
# Library unit tests
cargo test --lib --no-default-features

# Integration tests
cargo test --test blinky_logic_tests --no-default-features
cargo test --test mock_led_tests --no-default-features
cargo test --test property_tests --no-default-features
```

### Why `--no-default-features`?
The `embassy` feature is enabled by default for embedded builds. Tests must disable it to avoid compiling embedded dependencies on the host machine.

---

## Building (Embedded Target)

### Debug Build
```bash
cargo build --target thumbv7em-none-eabihf
```

### Release Build (Optimized)
```bash
cargo build --target thumbv7em-none-eabihf --release
```

---

## Flashing & Running (Hardware)

### Flash and Run with Debug Output
```bash
# Debug build
cargo run --target thumbv7em-none-eabihf

# Release build (recommended)
cargo run --target thumbv7em-none-eabihf --release
```

This will:
1. Build the firmware
2. Flash it to the connected nRF52 board
3. Start RTT session to view `defmt` logs

### Requirements
- nRF52 development board (e.g., nRF52840-DK)
- Debug probe connected (J-Link, CMSIS-DAP, or on-board debugger)

---

## Project Structure

```
active_note/
├── src/
│   ├── lib.rs          # Testable business logic (no_std when not testing)
│   └── main.rs         # Hardware-specific embedded binary
├── tests/
│   ├── blinky_logic_tests.rs   # Integration tests
│   ├── mock_led_tests.rs       # Mock hardware tests
│   └── property_tests.rs       # Property-based tests
├── .cargo/
│   └── config.toml     # Cargo configuration
├── Cargo.toml          # Dependencies & features
├── memory.x            # Memory layout
├── rust-toolchain.toml # Rust toolchain specification
└── build.rs            # Build script
```

---

## Features

### Default Feature: `embassy`
Includes all embedded dependencies:
- `embassy-executor` - Async runtime
- `embassy-time` - Time/timer abstractions  
- `embassy-nrf` - nRF52 HAL
- `cortex-m` / `cortex-m-rt` - ARM Cortex-M support
- `defmt` / `defmt-rtt` - Logging infrastructure
- `panic-probe` - Panic handler

### Building Without Features
```bash
# Library only, no embedded deps
cargo build --no-default-features
```

---

## Common Workflows

### Typical Development Cycle

1. **Write/modify logic** in `src/lib.rs`
   ```bash
   vim src/lib.rs
   ```

2. **Run tests** to verify correctness
   ```bash
   cargo test --no-default-features
   ```

3. **Build firmware** for target
   ```bash
   cargo build --target thumbv7em-none-eabihf --release
   ```

4. **Flash to hardware** and observe logs
   ```bash
   cargo run --target thumbv7em-none-eabihf --release
   ```

5. **Iterate** - Return to step 1

### Quick Validation
```bash
# Run all tests and build in one command
cargo test --no-default-features && \
cargo build --target thumbv7em-none-eabihf --release
```

---

## Chip Configuration

### Changing Target Chip

The project defaults to **nRF52840**. To target a different chip:

1. Update `Cargo.toml`:
   ```toml
   embassy-nrf = { 
       version = "0.3", 
       features = ["nrf52833", "time-driver-rtc1", "gpiote"],
       optional = true 
   }
   ```

2. Update `.cargo/config.toml`:
   ```toml
   [target.'cfg(all(target_arch = "arm", target_os = "none"))']
   runner = "probe-rs run --chip nRF52833_xxAA"
   ```

### Supported nRF52 Chips
- `nrf52805`
- `nrf52810` 
- `nrf52811`
- `nrf52820`
- `nrf52832`
- `nrf52833`
- `nrf52840` (default)

---

## Troubleshooting

### Tests Fail to Compile
**Problem**: Tests trying to compile embedded dependencies

**Solution**: Always use `--no-default-features` flag:
```bash
cargo test --no-default-features
```

### Binary Fails to Build
**Problem**: Missing target or configuration

**Solution**: Ensure you specify the target:
```bash
cargo build --target thumbv7em-none-eabihf
```

### Flash/Run Fails
**Problem**: Hardware not detected or probe-rs not installed

**Solutions**:
- Install probe-rs: `cargo install probe-rs-tools`
- Connect hardware via debug probe
- Check probe detection: `probe-rs list`
- Verify chip name in `.cargo/config.toml` matches your hardware

### Logs Not Appearing
**Problem**: RTT not working

**Solution**: 
- Ensure `DEFMT_LOG` environment variable is set (default: "debug")
- Check that `defmt-rtt` is initialized in `main.rs`
- Try different log levels: `DEFMT_LOG=trace cargo run ...`

---

## Test Coverage Summary

| Test Suite | Tests | Description |
|------------|-------|-------------|
| Unit tests (`lib.rs`) | 12 | Core logic functions |
| Integration tests | 12 | End-to-end scenarios |
| Mock tests | 13 | Hardware simulation |
| Property tests | 13 | Invariant verification |
| **Total** | **50** | **Complete coverage** |

All tests run on host machine without hardware requirements.

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DEFMT_LOG` | `debug` | Log level for defmt (trace, debug, info, warn, error) |

Example:
```bash
DEFMT_LOG=trace cargo run --target thumbv7em-none-eabihf --release
```

---

## Additional Resources

- [Embassy Documentation](https://embassy.dev/)
- [probe-rs Documentation](https://probe.rs/)
- [nRF52840 Product Specification](https://infocenter.nordicsemi.com/topic/struct_nrf52/struct/nrf52840.html)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

---

## Version Information

- **Rust Edition**: 2021
- **Embassy Version**: 0.7 (executor), 0.4 (time), 0.3 (nrf)
- **Cortex-M**: 0.7
- **Target**: `thumbv7em-none-eabihf` (ARMv7E-M with FPU)

---

*Last Updated: 2024*