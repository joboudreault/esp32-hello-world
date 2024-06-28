#![no_std]
#![no_main]

#[cfg(feature = "log-espflash")]
use esp_backtrace as _;
#[cfg(feature = "log-espflash")]
use esp_println as _;

// Others
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, timer::TimerGroup,
};

#[main]
async fn start(_spawner: Spawner) -> ! {
    // Take control of all peripherals
    let peripherals = Peripherals::take();

    // Initialize Embassy
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    #[allow(clippy::infinite_loop)]
    loop {
        defmt::info!("Here in main() !");
        Timer::after(Duration::from_millis(500)).await;
    }
}
