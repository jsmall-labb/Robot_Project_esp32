#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    gpio::{Io, Level, Output, OutputConfig},
    main,
    time::{Duration, Instant},
};
use esp_println::println;

// You need a panic handler. Usually, you you would use esp_backtrace, panic-probe, or
// something similar, but you can also bring your own like this:
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[main]
fn main() -> ! {
    esp_bootloader_esp_idf::esp_app_desc!();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
    loop {
        led.toggle();
        // Wait for half a second
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

}