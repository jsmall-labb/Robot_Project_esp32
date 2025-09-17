#![no_std]
#![no_main]

use core::hash::Hasher;
use esp32_hal::{
    clock::{ClockControl, Clocks},
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    uart::{
        config::{Config, DataBits, Parity, StopBits},
        Uart,
        TxRxPins,
    },
    Delay,

};
use esp_backtrace as _;
use nb::block;

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    uart_example(&mut delay, io, peripherals.UART0, &clocks);


}
fn uart_example(
    delay: &mut Delay,
    io: IO,
    uart_peripheral: esp32_hal::peripherals::UART0,
    clocks: &Clocks
) -> ! {
    let tx_pin = io.pins.gpio1;
    let rx_pin = io.pins.gpio3;

    let config = Config {
        baudrate: 115200,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1
    };

    let pins = TxRxPins::new_tx_rx(
        tx_pin.into_push_pull_output(),
        rx_pin.into_floating_input()
    );

    let mut uart = Uart::new_with_config(
        uart_peripheral,
        config,
        Some(pins),
        clocks
    );

    loop {
        uart_communication_loop(&mut uart, delay);
    }
}

fn uart_communication_loop(
    uart: &mut Uart<esp32_hal::peripherals::UART0>,
    delay: &mut Delay
) {
    Ok(uart.write_str("Hello World! from ESP32")).expect("Failed to write");

    loop{
        match uart.read() {
            Ok(byte) => {
                block!(uart.write(byte)).expect("Failed to echo byte");

                if(byte == b'\r' || byte == b'\n'){
                    Ok(uart.write_str("\r\n")).expect("Failed to write newline");
                }

                if byte == b'q'{
                    Ok(uart.write_str("Exiting\r\n")).expect("Failed to write Exit");
                }
            }
            Err(nb::Error::WouldBlock) => {}
            Err(_) => {
                Ok(uart.write_str("Error\r\n")).expect("Failed to write Error");
            }
        }
        delay.delay_ms(10u32);
    }
    delay.delay_ms(1000u32);
}

