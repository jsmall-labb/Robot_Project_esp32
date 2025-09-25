#![no_std]
#![no_main]

use code::motor_controller::{Motor, MotorController};


use esp_hal::{
    clock::CpuClock,
    gpio::{Io, Level, Output, OutputConfig, OutputPin},
    peripherals::Peripherals,
    main,
    time::{Duration, Instant, Rate},
    mcpwm::{
        operator::{PwmPinConfig, PwmActions, PwmUpdateMethod},
        timer::{PwmWorkingMode},
        PeripheralClockConfig, McPwm,
    },
};
use esp_println::println;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    esp_hal::system::software_reset()
}

#[main]
fn main() -> ! {
    esp_bootloader_esp_idf::esp_app_desc!();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut _robot_controller = setup_robot(peripherals);

    loop {
        _robot_controller.forward(100);
        _robot_controller.brake();
        _robot_controller.turn_left(100);
        _robot_controller.brake();
        _robot_controller.turn_right(100);
        _robot_controller.stop();

    }
}

pub fn setup_robot(peripherals: Peripherals)  -> MotorController<
    esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0<'static>, 0, true>,
    esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0<'static>, 1, true>,
    esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0<'static>, 2, true>,
    esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM1<'static>, 0, true>,
    >{
    let clock_cfg = PeripheralClockConfig::with_frequency(Rate::from_mhz(40)).unwrap();
    let mut mcpwm1 = McPwm::new(peripherals.MCPWM0, clock_cfg);
    let mut mcpwm2 = McPwm::new(peripherals.MCPWM1, clock_cfg);

    mcpwm1.operator0.set_timer(&mcpwm1.timer0);
    mcpwm1.operator1.set_timer(&mcpwm1.timer0);
    mcpwm1.operator2.set_timer(&mcpwm1.timer0);
    mcpwm2.operator0.set_timer(&mcpwm2.timer0);


    let pwm1 = mcpwm1.operator0.with_pin_a(peripherals.GPIO12, PwmPinConfig::UP_ACTIVE_HIGH);
    let pwm2 = mcpwm1.operator1.with_pin_a(peripherals.GPIO13, PwmPinConfig::UP_ACTIVE_HIGH);
    let pwm3 = mcpwm1.operator2.with_pin_a(peripherals.GPIO18, PwmPinConfig::UP_ACTIVE_HIGH);
    let pwm4 = mcpwm2.operator0.with_pin_a(peripherals.GPIO19, PwmPinConfig::UP_ACTIVE_HIGH);

    let motor1 = Motor::new(pwm1, peripherals.GPIO14, peripherals.GPIO15);
    let motor2 = Motor::new(pwm2, peripherals.GPIO16, peripherals.GPIO17);
    let motor3 = Motor::new(pwm3, peripherals.GPIO21, peripherals.GPIO22);
    let motor4 = Motor::new(pwm4, peripherals.GPIO23, peripherals.GPIO25);

    MotorController::new(motor1, motor2, motor3, motor4)
}