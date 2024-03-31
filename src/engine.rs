use crate::motor::Engine;
use esp_idf_hal::gpio::{Input, InputPin, PinDriver};
use esp_idf_hal::ledc::LedcDriver;
use esp_idf_hal::sys::EspError;

fn remap(value: u32, min: u8, max: u32) -> u32 {
    let percentage = value as f32 / 100.0;
    let min = min as f32;
    let max = max as f32;

    (percentage * (max - min) + min) as u32
}

pub struct HybridEngine<'d, FAULT: InputPin> {
    eep: Option<LedcDriver<'d>>,
    fault: Option<PinDriver<'d, FAULT, Input>>,
    minimum_force: u8,
}

impl<'d, FAULT: InputPin> Engine for HybridEngine<'d, FAULT> {
    fn enable(&mut self) -> Result<(), EspError> {
        if let Some(eep) = &mut self.eep {
            eep.enable()?;
        }

        Ok(())
    }

    fn disable(&mut self) -> Result<(), EspError> {
        if let Some(eep) = &mut self.eep {
            eep.disable()?;
        }

        Ok(())
    }

    fn set_force(&mut self, force: u8) -> Result<(), EspError> {
        if let Some(eep) = &mut self.eep {
            let max_duty = eep.get_max_duty();

            let percentage = match force {
                0 => 0,
                force => remap(force as u32, self.minimum_force, max_duty),
            };

            eep.set_duty(percentage)
        } else {
            Ok(())
        }
    }

    fn set_min_force(&mut self, force: u8) {
        self.minimum_force = force;
    }

    fn is_faulty(&self) -> bool {
        if let Some(fault) = &self.fault {
            fault.is_high()
        } else {
            false
        }
    }
}

impl<'d, FAULT: InputPin> HybridEngine<'d, FAULT> {
    pub fn new(eep: Option<LedcDriver<'d>>, fault: Option<PinDriver<'d, FAULT, Input>>) -> HybridEngine<'d, FAULT> {
        Self {
            eep,
            fault,
            minimum_force: 0,
        }
    }
}
