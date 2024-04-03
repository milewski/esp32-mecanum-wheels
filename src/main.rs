#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use drv8833_driver::driver::{DRV8833Driver, MotorDriver, MotorDriverPwm};
use embedded_hal::digital::InputPin;
use esp_idf_hal::gpio::{AnyInputPin, Input, PinDriver};
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::sys::EspError;

use crate::motor::Engine;

mod car;
mod engine;
mod hybrid_motor;
mod motor;
mod wheel;

enum JoystickSide {
    Left,
    Right,
}

impl From<u8> for JoystickSide {
    fn from(value: u8) -> Self {
        match value {
            0 => JoystickSide::Left,
            _ => JoystickSide::Right,
        }
    }
}

fn main() {
    let peripherals = Peripherals::take().expect("unable to take peripherals...");

    let in1 = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let in2 = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let in3 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    let in4 = PinDriver::output(peripherals.pins.gpio19).unwrap();
    // let sleep = PinDriver::output(peripherals.pins.gpio3).unwrap();

    let timer = LedcTimerDriver::new(peripherals.ledc.timer3, &TimerConfig::default()).unwrap();
    let pwm = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio3).unwrap();

    // let pwm = Arc::new(Mutex::new(pwm));
    //
    let mut motor = DRV8833Driver::new_pwm_sync(
        in1, in2, in3, in4, pwm, None::<PinDriver<AnyInputPin, Input>>
    );


    motor.forward(70).unwrap();
    // motor.sleep()?;

}
