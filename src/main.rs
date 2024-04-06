#![allow(dead_code)]

use drv8833_driver::{MotorDriver, PwmMovement};

use esp32_nimble::{uuid128, BLEAdvertisementData, BLEDevice, NimbleProperties};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{AnyInputPin, Input, PinDriver};
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::sys::EspError;

use crate::car::Mecanum;

mod car;

#[derive(Debug)]
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

    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &TimerConfig::default()).unwrap();

    let a_in1 = PinDriver::output(peripherals.pins.gpio5).unwrap();
    let a_in2 = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let a_in3 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    let a_in4 = PinDriver::output(peripherals.pins.gpio19).unwrap();

    let b_in1 = PinDriver::output(peripherals.pins.gpio9).unwrap();
    let b_in2 = PinDriver::output(peripherals.pins.gpio8).unwrap();
    let b_in3 = PinDriver::output(peripherals.pins.gpio7).unwrap();
    let b_in4 = PinDriver::output(peripherals.pins.gpio6).unwrap();

    let a_pwm = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio3).unwrap();
    let b_pwm = LedcDriver::new(peripherals.ledc.channel1, &timer, peripherals.pins.gpio2).unwrap();

    let motor_a = MotorDriver::new_pwm_split_single(
        a_in1, a_in2, a_in3, a_in4, a_pwm, None::<PinDriver<AnyInputPin, Input>>,
    );

    let motor_b = MotorDriver::new_pwm_split_single(
        b_in1, b_in2, b_in3, b_in4, b_pwm, None::<PinDriver<AnyInputPin, Input>>,
    );

    let mut car = Mecanum::new(motor_a, motor_b);

    let device = BLEDevice::take();
    let server = device.get_server();
    let advertising = device.get_advertising();

    let service_id = uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa");
    let characteristic_id = uuid128!("a3c87500-8ed3-4bdf-8a39-a01bebede295");

    server.on_connect(|_, connection| println!("Client connected: {:?}", connection));

    let on_disconnect_car = car.clone();

    server.on_disconnect(move |connection, _| {
        if let Ok(mut car) = on_disconnect_car.lock() {
            if let Err(error) = car.stop() {
                println!("Failed to stop with the following error: {:?}", error);
            }
        }

        println!("Client disconnected: {:?}", connection)
    });

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
        FreeRtos::delay_ms(10);
    }
}
