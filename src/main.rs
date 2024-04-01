#![allow(dead_code)]

use std::fmt::{Debug, Error};
use std::sync::Mutex;
use drv8833_driver::driver::{DRV8833Driver, MotorDriver, MotorDriverError};
use drv8833_driver::parallel_driver::ParallelDriver;
use embedded_hal::digital::InputPin;

use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{AnyInputPin, AnyIOPin, Gpio0, Gpio10, Gpio18, Gpio19, Gpio2, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, GpioError, Input, Output, PinDriver};
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::sys::EspError;

use crate::car::Mecanum;
use crate::motor::Engine;
use crate::motor::{MotorFactory};

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

fn main() -> Result<(), EspError> {
    let peripherals = Peripherals::take().expect("unable to take peripherals...");

    let in1 = PinDriver::output(peripherals.pins.gpio5)?;
    let in2 = PinDriver::output(peripherals.pins.gpio4)?;
    let in3 = PinDriver::output(peripherals.pins.gpio18)?;
    let in4 = PinDriver::output(peripherals.pins.gpio19)?;
    // let sleep = PinDriver::output(peripherals.pins.gpio3)?;
    let fault = PinDriver::input(peripherals.pins.gpio10)?;

    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default())?;
    let pwm = LedcDriver::new(peripherals.ledc.channel0, timer, peripherals.pins.gpio3)?;

    // let in1_b = PinDriver::output(peripherals.pins.gpio9)?;
    // let in2_b = PinDriver::output(peripherals.pins.gpio8)?;
    // let in3_b = PinDriver::output(peripherals.pins.gpio7)?;
    // let in4_b = PinDriver::output(peripherals.pins.gpio6)?;
    // let sleep_b = PinDriver::output(peripherals.pins.gpio2)?;
    // let fault_b = PinDriver::input(peripherals.pins.gpio0)?;

    let mut motor_a = DRV8833Driver::new_pwm_single(in1, in2, in3, in4, pwm, Some(fault));
    // let mut motor_a = DRV8833Driver::new_parallel(in1, in2, in3, in4, Some(sleep), Some(fault));

    // motor_a.wakeup().unwrap();

    loop {
        // println!("{:?}", motor_a.is_fault());

        motor_a.a.forward().unwrap();

        FreeRtos::delay_ms(200);

        FreeRtos::delay_ms(200);
        motor_a.a.stop().unwrap();

        FreeRtos::delay_ms(100);
        motor_a.a.forward().unwrap();

        FreeRtos::delay_ms(100);
    }

    // let car = {
    //     let mut motor_a = MotorFactory::new_hybrid(
    //         peripherals.ledc.channel1,
    //         peripherals.ledc.timer0,
    //         peripherals.pins.gpio5,
    //         peripherals.pins.gpio4,
    //         peripherals.pins.gpio18,
    //         peripherals.pins.gpio19,
    //         Some(peripherals.pins.gpio3),
    //         None::<AnyInputPin>,
    //     )?;
    //
    //     let mut motor_b = MotorFactory::new_hybrid(
    //         peripherals.ledc.channel2,
    //         peripherals.ledc.timer1,
    //         peripherals.pins.gpio9,
    //         peripherals.pins.gpio8,
    //         peripherals.pins.gpio7,
    //         peripherals.pins.gpio6,
    //         Some(peripherals.pins.gpio2),
    //         None::<AnyInputPin>,
    //     )?;
    //
    //     motor_a.engine.set_min_force(100);
    //     motor_b.engine.set_min_force(100);
    //
    //     Mutex::new(Mecanum::new(motor_a, motor_b))
    // };
    //
    // let device = BLEDevice::take();
    // let server = device.get_server();
    // let advertising = device.get_advertising();
    //
    // let service_id = uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa");
    // let characteristic_id = uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");
    //
    // server.on_connect(|_, connection| println!("Client connected: {:?}", connection));
    // server.on_disconnect(|connection, _| println!("Client disconnected: {:?}", connection));
    //
    // let service = server.create_service(service_id);
    // let characteristic = service
    //     .lock()
    //     .create_characteristic(characteristic_id, NimbleProperties::WRITE);
    //
    // characteristic.lock().on_write(move |arguments| {
    //     let data = arguments.recv_data();
    //     let side: JoystickSide = data[0].into();
    //     let x = data[1];
    //     let y = data[2];
    //
    //     if let Ok(mut car) = car.lock() {
    //         let _ = match side {
    //             JoystickSide::Left => car.spin(x, y),
    //             JoystickSide::Right => car.update(x, y),
    //         };
    //     }
    // });
    //
    // advertising
    //     .lock()
    //     .set_data(BLEAdvertisementData::new().add_service_uuid(service_id))
    //     .expect("failed to set advertisement data...");
    //
    // advertising.lock().start().expect("failed to start advertisement...");

    loop {
        FreeRtos::delay_ms(1);
    }
}
