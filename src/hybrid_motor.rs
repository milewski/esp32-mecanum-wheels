use esp_idf_hal::gpio::{Input, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::ledc::LedcDriver;

use crate::engine::HybridEngine;
use crate::wheel::StaticWheel;

pub struct HybridMotor<'d, IN1, IN2, IN3, IN4, FAULT>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        FAULT: InputPin,
{
    pub wheel_a: StaticWheel<'d, IN1, IN2>,
    pub wheel_b: StaticWheel<'d, IN3, IN4>,
    pub engine: HybridEngine<'d, FAULT>,
}

impl<'d, IN1, IN2, IN3, IN4, FAULT> HybridMotor<'d, IN1, IN2, IN3, IN4, FAULT>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        FAULT: InputPin,
{
    pub fn new(
        int1: PinDriver<'d, IN1, Output>,
        int2: PinDriver<'d, IN2, Output>,
        int3: PinDriver<'d, IN3, Output>,
        int4: PinDriver<'d, IN4, Output>,
        eep: Option<LedcDriver<'d>>,
        fault: Option<PinDriver<'d, FAULT, Input>>,
    ) -> Self {
        Self {
            wheel_a: StaticWheel::new(int1, int2),
            wheel_b: StaticWheel::new(int3, int4),
            engine: HybridEngine::new(eep, fault),
        }
    }
}
