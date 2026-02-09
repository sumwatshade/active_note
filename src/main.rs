#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// Import types from the library
use active_note::{BlinkyConfig, BlinkyPattern, BlinkyState};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    let config = BlinkyConfig::default();
    let mut pattern = BlinkyPattern::new(config);

    info!("Blinky started!");
    info!(
        "Config: ON={}ms OFF={}ms",
        config.on_duration_ms, config.off_duration_ms
    );

    loop {
        let (state, duration) = pattern.next();

        match state {
            BlinkyState::On => {
                info!("LED on (cycle #{})", pattern.cycle_count());
                led.set_high();
            }
            BlinkyState::Off => {
                info!("LED off");
                led.set_low();
            }
        }

        Timer::after_millis(duration as u64).await;
    }
}
