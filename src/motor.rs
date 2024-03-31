use crate::engine::HybridEngine;
use esp_idf_hal::gpio::{InputPin, OutputPin, PinDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::sys::EspError;

use crate::hybrid_motor::HybridMotor;
use crate::wheel::StaticWheel;

pub enum Vector {
    Backward,
    Forward,
}

pub trait Engine {
    fn enable(&mut self) -> Result<(), EspError>;
    fn disable(&mut self) -> Result<(), EspError>;
    fn set_force(&mut self, force: u8) -> Result<(), EspError>;
    fn set_min_force(&mut self, force: u8);
    fn is_faulty(&self) -> bool;
}

pub struct MotorFactory;

impl MotorFactory {
    pub fn new_hybrid<'d, CHANNEL1, TIMER, INT1, INT2, INT3, INT4, EEP, FAULT>(
        channel1: impl Peripheral<P=CHANNEL1> + 'd,
        timer: impl Peripheral<P=TIMER> + 'd,
        int1: impl Peripheral<P=INT1> + 'd,
        int2: impl Peripheral<P=INT2> + 'd,
        int3: impl Peripheral<P=INT3> + 'd,
        int4: impl Peripheral<P=INT4> + 'd,
        eep: Option<impl Peripheral<P=EEP> + 'd>,
        fault: Option<impl Peripheral<P=FAULT> + 'd>,
    ) -> Result<HybridMotor<'d, INT1, INT2, INT3, INT4, FAULT>, EspError>
        where
            CHANNEL1: LedcChannel,
            TIMER: LedcTimer,
            INT1: OutputPin,
            INT2: OutputPin,
            INT3: OutputPin,
            INT4: OutputPin,
            EEP: OutputPin,
            FAULT: InputPin,
    {
        let config = TimerConfig::default().frequency(50.kHz().into());
        let timer = LedcTimerDriver::new(timer, &config)?;

        let int1_driver = PinDriver::output(int1)?;
        let int2_driver = PinDriver::output(int2)?;
        let int3_driver = PinDriver::output(int3)?;
        let int4_driver = PinDriver::output(int4)?;

        let wheel_a = StaticWheel::new(int1_driver, int2_driver);
        let wheel_b = StaticWheel::new(int3_driver, int4_driver);

        let eep = eep.map(|eep| LedcDriver::new(channel1, &timer, eep)).transpose()?;
        let fault = fault.map(PinDriver::input).transpose()?;

        let engine = HybridEngine::new(eep, fault);

        Ok(HybridMotor {
            engine,
            wheel_a,
            wheel_b,
        })
    }
}

pub trait Wheel {
    fn update(&mut self, direction: &Vector, force: u8) -> Result<(), EspError>;
}

pub trait Motor {
    type Engine: Engine;
    type WheelA: Wheel;
    type WheelB: Wheel;

    fn start(&mut self) -> Result<(), EspError>;

    fn sleep(&mut self) -> Result<(), EspError>;

    fn update(&mut self, direction: &Vector, force: u8) -> Result<(), EspError>;

    fn split(self) -> (Self::Engine, Self::WheelA, Self::WheelB);
}
