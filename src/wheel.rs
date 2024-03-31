use crate::motor::{Vector, Wheel};
use esp_idf_hal::gpio::{Output, OutputPin, PinDriver};
use esp_idf_hal::sys::EspError;

pub struct StaticWheel<'d, IN1: OutputPin, IN2: OutputPin> {
    int1: PinDriver<'d, IN1, Output>,
    int2: PinDriver<'d, IN2, Output>,
}

impl<'d, IN1: OutputPin, IN2: OutputPin> Wheel for StaticWheel<'d, IN1, IN2> {
    fn update(&mut self, direction: &Vector, force: u8) -> Result<(), EspError> {
        if force == 0 {
            self.int1.set_low()?;
            self.int2.set_low()?;

            return Ok(());
        }

        match direction {
            Vector::Backward => {
                self.int1.set_high()?;
                self.int2.set_low()?;
            }
            Vector::Forward => {
                self.int1.set_low()?;
                self.int2.set_high()?;
            }
        }

        Ok(())
    }
}

impl<'d, IN1: OutputPin, IN2: OutputPin> StaticWheel<'d, IN1, IN2> {
    pub fn new(int1: PinDriver<'d, IN1, Output>, int2: PinDriver<'d, IN2, Output>) -> StaticWheel<'d, IN1, IN2> {
        Self { int1, int2 }
    }
}
