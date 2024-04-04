#![allow(dead_code)]

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use drv8833_driver::driver::{Driver, DRV8833Driver, MotorDriver, Movement};
use embedded_hal::digital::InputPin;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{AnyInputPin, Gpio18, Gpio19, Gpio3, Gpio4, Gpio5, Input, Output, OutputPin, PinDriver};
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::sys::EspError;
use crate::motor::Engine;
use drv8833_driver::driver::PwmMovement;
use drv8833_driver::driver::Breaks;
use drv8833_driver::parallel_driver::ParallelDriver;

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

    //
    // NEW_PWM_SPLIT
    //
    // let sleep = PinDriver::output(peripherals.pins.gpio3).unwrap();
    // let timer = LedcTimerDriver::new(peripherals.ledc.timer3, &TimerConfig::default()).unwrap();
    //
    // let in1 = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio5).unwrap();
    // let in2 = LedcDriver::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio4).unwrap();
    // let in3 = LedcDriver::new(peripherals.ledc.channel2, &timer, peripherals.pins.gpio18).unwrap();
    // let in4 = LedcDriver::new(peripherals.ledc.channel3, &timer, peripherals.pins.gpio19).unwrap();
    //
    // let mut motor = DRV8833Driver::new_pwm_split(
    //     in1, in2, in3, in4, Some(sleep), None::<PinDriver<AnyInputPin, Input>>,
    // );
    //
    // motor.wakeup().unwrap();
    //
    // loop {
    //     motor.a.forward(50).unwrap();
    //     motor.b.forward(50).unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.reverse(50).unwrap();
    //     motor.b.reverse(50).unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.coast().unwrap();
    //     motor.b.coast().unwrap();
    //     FreeRtos::delay_ms(1000);
    //
    //     motor.a.forward(100).unwrap();
    //     motor.b.forward(100).unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.reverse(100).unwrap();
    //     motor.b.reverse(100).unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.stop().unwrap();
    //     motor.b.stop().unwrap();
    //     FreeRtos::delay_ms(1000);
    // }

    //
    // NEW_PWM_PARALLEL
    //
    let in1 = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let in2 = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let in3 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    let in4 = PinDriver::output(peripherals.pins.gpio19).unwrap();

    let timer = LedcTimerDriver::new(peripherals.ledc.timer3, &TimerConfig::default()).unwrap();
    let sleep = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio3).unwrap();

    let mut motor = MotorDriver::new_pwm_parallel(
        in1, in2, in3, in4, sleep, None::<PinDriver<AnyInputPin, Input>>,
    );

    motor.set_min_duty(150);

    loop {
        motor.forward(50).unwrap();
        FreeRtos::delay_ms(1000);
        motor.reverse(50).unwrap();
        FreeRtos::delay_ms(1000);
        motor.coast().unwrap();
        FreeRtos::delay_ms(1000);

        motor.forward(100).unwrap();
        FreeRtos::delay_ms(1000);
        motor.reverse(100).unwrap();
        FreeRtos::delay_ms(1000);
        motor.stop().unwrap();
        FreeRtos::delay_ms(1000);
    }

    //
    // Parallel
    //
    // let in1 = PinDriver::output(peripherals.pins.gpio5).unwrap();
    // let in2 = PinDriver::output(peripherals.pins.gpio4).unwrap();
    // let in3 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    // let in4 = PinDriver::output(peripherals.pins.gpio19).unwrap();
    // let sleep = PinDriver::output(peripherals.pins.gpio3).unwrap();
    //
    // let mut motor = DRV8833Driver::new_parallel(
    //     in1, in2, in3, in4, Some(sleep), None::<PinDriver<AnyInputPin, Input>>,
    // );
    // motor.wakeup().unwrap();
    // play_sequence(&mut motor);

    //
    // Split
    //
    // let in1 = PinDriver::output(peripherals.pins.gpio5).unwrap();
    // let in2 = PinDriver::output(peripherals.pins.gpio4).unwrap();
    // let in3 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    // let in4 = PinDriver::output(peripherals.pins.gpio19).unwrap();
    // let sleep = PinDriver::output(peripherals.pins.gpio3).unwrap();
    //
    // let mut motor = DRV8833Driver::new_split(
    //     in1, in2, in3, in4, Some(sleep), None::<PinDriver<AnyInputPin, Input>>,
    // );
    //
    // motor.wakeup().unwrap();
    //
    // loop {
    //     motor.a.forward().unwrap();
    //     motor.b.forward().unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.reverse().unwrap();
    //     motor.b.reverse().unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.coast().unwrap();
    //     motor.b.coast().unwrap();
    //     FreeRtos::delay_ms(1000);
    //
    //     motor.a.forward().unwrap();
    //     motor.b.forward().unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.reverse().unwrap();
    //     motor.b.reverse().unwrap();
    //     FreeRtos::delay_ms(1000);
    //     motor.a.stop().unwrap();
    //     motor.b.stop().unwrap();
    //     FreeRtos::delay_ms(1000);
    // }
}
