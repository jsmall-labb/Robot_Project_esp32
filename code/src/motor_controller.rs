#![no_std]


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

pub struct Motor<PWM> {
    enable_pwm: PWM,
    input_1: Output<'static>,
    input_2: Output<'static>,
}

impl<PWM> Motor<PWM> {
    pub fn new(pwm_pin: PWM, in1: impl OutputPin + 'static, in2: impl OutputPin + 'static) -> Self {
        Self {
            enable_pwm: pwm_pin,
            input_1: Output::new(in1, Level::Low, OutputConfig::default()),
            input_2: Output::new(in2, Level::Low, OutputConfig::default()),
        }
    }

    pub fn emergency_stop(&mut self)
    where
        PWM: embedded_hal::pwm::SetDutyCycle,
    {
        self.input_1.set_low();
        self.input_2.set_low();
        let _ = self.enable_pwm.set_duty_cycle_percent(0);
    }

    pub fn forward(&mut self, speed: u8)
    where
        PWM: embedded_hal::pwm::SetDutyCycle,
    {
        self.input_1.set_high();
        self.input_2.set_low();
        let _ = self.enable_pwm.set_duty_cycle_percent(speed.min(100));
    }

    pub fn backward(&mut self, speed: u8)
    where
        PWM: embedded_hal::pwm::SetDutyCycle,
    {
        self.input_1.set_low();
        self.input_2.set_high();
        let _ = self.enable_pwm.set_duty_cycle_percent(speed.min(100));
    }

    pub fn stop(&mut self)
    where
        PWM: embedded_hal::pwm::SetDutyCycle,
    {
        let _ = self.enable_pwm.set_duty_cycle_percent(0);
    }

    pub fn brake(&mut self)
    where
        PWM: embedded_hal::pwm::SetDutyCycle,
    {
        self.input_1.set_high();
        self.input_2.set_high();
        let _ = self.enable_pwm.set_duty_cycle_percent(100);
    }
}

pub struct MotorController<P1, P2, P3, P4> {
    motor_1: Motor<P1>,
    motor_2: Motor<P2>,
    motor_3: Motor<P3>,
    motor_4: Motor<P4>,
}

impl <P1, P2, P3, P4> MotorController<P1, P2, P3, P4>
where
    P1: embedded_hal::pwm::SetDutyCycle,
    P2: embedded_hal::pwm::SetDutyCycle,
    P3: embedded_hal::pwm::SetDutyCycle,
    P4: embedded_hal::pwm::SetDutyCycle,
{
    pub fn new(motor1: Motor<P1>, motor2: Motor<P2>, motor3: Motor<P3>, motor4: Motor<P4>) -> Self {
        Self {
            motor_1: motor1,
            motor_2: motor2,
            motor_3: motor3,
            motor_4: motor4,
        }
    }

    pub fn forward(&mut self, speed: u8) {
        self.motor_1.forward(speed);
        self.motor_2.forward(speed);
        self.motor_3.forward(speed);
        self.motor_4.forward(speed);
    }

    pub fn backward(&mut self, speed: u8) {
        self.motor_1.backward(speed);
        self.motor_2.backward(speed);
        self.motor_3.backward(speed);
        self.motor_4.backward(speed);
    }

    pub fn turn_left(&mut self, speed: u8) {
        self.motor_1.backward(speed);
        self.motor_2.backward(speed);
        self.motor_3.forward(speed);
        self.motor_4.forward(speed);
    }

    pub fn turn_right(&mut self, speed: u8) {
        self.motor_1.forward(speed);
        self.motor_2.forward(speed);
        self.motor_3.backward(speed);
        self.motor_4.backward(speed);
    }

    pub fn stop(&mut self) {
        self.motor_1.stop();
        self.motor_2.stop();
        self.motor_3.stop();
        self.motor_4.stop();
    }

    pub fn brake(&mut self) {
        self.motor_1.brake();
        self.motor_2.brake();
        self.motor_3.brake();
        self.motor_4.brake();
    }

    pub fn emergency_stop(&mut self) {
        self.motor_1.emergency_stop();
        self.motor_2.emergency_stop();
        self.motor_3.emergency_stop();
        self.motor_4.emergency_stop();
    }
}