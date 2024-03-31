#![allow(dead_code)]

use std::sync::Mutex;

use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyInputPin;
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

    let car = {
        let mut motor_a = MotorFactory::new_hybrid(
            peripherals.ledc.channel1,
            peripherals.ledc.timer0,
            peripherals.pins.gpio5,
            peripherals.pins.gpio4,
            peripherals.pins.gpio18,
            peripherals.pins.gpio19,
            Some(peripherals.pins.gpio3),
            None::<AnyInputPin>,
        )?;

        let mut motor_b = MotorFactory::new_hybrid(
            peripherals.ledc.channel2,
            peripherals.ledc.timer1,
            peripherals.pins.gpio9,
            peripherals.pins.gpio8,
            peripherals.pins.gpio7,
            peripherals.pins.gpio6,
            Some(peripherals.pins.gpio2),
            None::<AnyInputPin>,
        )?;

        motor_a.engine.set_min_force(100);
        motor_b.engine.set_min_force(100);

        Mutex::new(Mecanum::new(motor_a, motor_b))
    };

    let device = BLEDevice::take();
    let server = device.get_server();
    let advertising = device.get_advertising();

    let service_id = uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa");
    let characteristic_id = uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");

    server.on_connect(|_, connection| println!("Client connected: {:?}", connection));
    server.on_disconnect(|connection, _| println!("Client disconnected: {:?}", connection));

    let service = server.create_service(service_id);
    let characteristic = service
        .lock()
        .create_characteristic(characteristic_id, NimbleProperties::WRITE);

    characteristic.lock().on_write(move |arguments| {
        let data = arguments.recv_data();
        let side: JoystickSide = data[0].into();
        let x = data[1];
        let y = data[2];

        if let Ok(mut car) = car.lock() {
            let _ = match side {
                JoystickSide::Left => car.spin(x, y),
                JoystickSide::Right => car.update(x, y),
            };
        }
    });

    advertising
        .lock()
        .set_data(BLEAdvertisementData::new().add_service_uuid(service_id))
        .expect("failed to set advertisement data...");

    advertising.lock().start().expect("failed to start advertisement...");

    loop {
        FreeRtos::delay_ms(1);
    }
}
