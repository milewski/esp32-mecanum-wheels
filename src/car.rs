use std::f64::consts::PI;

use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::sys::EspError;

use crate::hybrid_motor::HybridMotor;
use crate::motor::{Engine, Vector, Wheel};

pub struct Mecanum<'d, INA1, INA2, INA3, INA4, FaultA, INB1, INB2, INB3, INB4, FaultB>
    where
        INA1: OutputPin,
        INA2: OutputPin,
        INA3: OutputPin,
        INA4: OutputPin,
        FaultA: InputPin,
        INB1: OutputPin,
        INB2: OutputPin,
        INB3: OutputPin,
        INB4: OutputPin,
        FaultB: InputPin,
{
    motor_a: HybridMotor<'d, INA1, INA2, INA3, INA4, FaultA>,
    motor_b: HybridMotor<'d, INB1, INB2, INB3, INB4, FaultB>,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}

impl<'d, INA1, INA2, INA3, INA4, FaultA, INB1, INB2, INB3, INB4, FaultB> Mecanum<'d, INA1, INA2, INA3, INA4, FaultA, INB1, INB2, INB3, INB4, FaultB>
    where
        INA1: OutputPin,
        INA2: OutputPin,
        INA3: OutputPin,
        INA4: OutputPin,
        FaultA: InputPin,
        INB1: OutputPin,
        INB2: OutputPin,
        INB3: OutputPin,
        INB4: OutputPin,
        FaultB: InputPin,
{
    pub fn new(
        motor_a: HybridMotor<'d, INA1, INA2, INA3, INA4, FaultA>,
        motor_b: HybridMotor<'d, INB1, INB2, INB3, INB4, FaultB>,
    ) -> Self {
        Self { motor_a, motor_b }
    }

    pub fn spin(&mut self, x: u8, y: u8) -> Result<(), EspError> {
        let (direction, force) = self.interpret_direction_and_force(x, y);

        self.set_force(force)?;

        match direction {
            Direction::Left => self.spin_left(force)?,
            Direction::Right => self.spin_right(force)?,
            _ => {}
        }

        Ok(())
    }

    pub fn update(&mut self, x: u8, y: u8) -> Result<(), EspError> {
        let (direction, force) = self.interpret_direction_and_force(x, y);

        self.set_force(force)?;

        match direction {
            Direction::Up => self.up(force)?,
            Direction::Down => self.down(force)?,
            Direction::Left => self.left(force)?,
            Direction::Right => self.right(force)?,
            Direction::TopRight => self.top_right(force)?,
            Direction::TopLeft => self.top_left(force)?,
            Direction::BottomLeft => self.bottom_left(force)?,
            Direction::BottomRight => self.bottom_right(force)?,
        }

        Ok(())
    }

    pub fn right(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Forward, force)?;
        self.motor_a.wheel_b.update(&Vector::Backward, force)?;

        self.motor_b.wheel_a.update(&Vector::Backward, force)?;
        self.motor_b.wheel_b.update(&Vector::Forward, force)?;

        Ok(())
    }

    pub fn left(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Backward, force)?;
        self.motor_a.wheel_b.update(&Vector::Forward, force)?;

        self.motor_b.wheel_a.update(&Vector::Forward, force)?;
        self.motor_b.wheel_b.update(&Vector::Backward, force)?;

        Ok(())
    }

    pub fn up(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Forward, force)?;
        self.motor_a.wheel_b.update(&Vector::Forward, force)?;

        self.motor_b.wheel_a.update(&Vector::Forward, force)?;
        self.motor_b.wheel_b.update(&Vector::Forward, force)?;

        Ok(())
    }

    pub fn down(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Backward, force)?;
        self.motor_a.wheel_b.update(&Vector::Backward, force)?;

        self.motor_b.wheel_a.update(&Vector::Backward, force)?;
        self.motor_b.wheel_b.update(&Vector::Backward, force)?;

        Ok(())
    }

    pub fn top_right(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Forward, force)?;
        self.motor_a.wheel_b.update(&Vector::Forward, 0)?;

        self.motor_b.wheel_a.update(&Vector::Forward, 0)?;
        self.motor_b.wheel_b.update(&Vector::Forward, force)?;

        Ok(())
    }

    pub fn top_left(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Forward, 0)?;
        self.motor_a.wheel_b.update(&Vector::Forward, force)?;

        self.motor_b.wheel_a.update(&Vector::Forward, force)?;
        self.motor_b.wheel_b.update(&Vector::Forward, 0)?;

        Ok(())
    }

    pub fn bottom_left(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Backward, force)?;
        self.motor_a.wheel_b.update(&Vector::Backward, 0)?;

        self.motor_b.wheel_a.update(&Vector::Backward, 0)?;
        self.motor_b.wheel_b.update(&Vector::Backward, force)?;

        Ok(())
    }

    pub fn bottom_right(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Backward, 0)?;
        self.motor_a.wheel_b.update(&Vector::Backward, force)?;

        self.motor_b.wheel_a.update(&Vector::Backward, force)?;
        self.motor_b.wheel_b.update(&Vector::Backward, 0)?;

        Ok(())
    }

    pub fn spin_right(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Forward, force)?;
        self.motor_a.wheel_b.update(&Vector::Backward, force)?;

        self.motor_b.wheel_a.update(&Vector::Forward, force)?;
        self.motor_b.wheel_b.update(&Vector::Backward, force)?;

        Ok(())
    }

    pub fn spin_left(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.wheel_a.update(&Vector::Backward, force)?;
        self.motor_a.wheel_b.update(&Vector::Forward, force)?;

        self.motor_b.wheel_a.update(&Vector::Backward, force)?;
        self.motor_b.wheel_b.update(&Vector::Forward, force)?;

        Ok(())
    }

    fn set_force(&mut self, force: u8) -> Result<(), EspError> {
        self.motor_a.engine.set_force(force)?;
        self.motor_b.engine.set_force(force)?;

        Ok(())
    }

    fn interpret_direction_and_force(&self, x: u8, y: u8) -> (Direction, u8) {
        let center_x = 127;
        let center_y = 127;

        let x_diff = x as i16 - center_x as i16;
        let y_diff = y as i16 - center_y as i16;

        // Calculate the angle
        let angle_rad = (x_diff as f64).atan2(y_diff as f64);
        let angle_deg = (angle_rad * 180.0 / PI).round();

        // Calculate the magnitude (force)
        let force = ((x_diff.pow(2) + y_diff.pow(2)) as f64).sqrt() as u8;

        let normalized_degree = (angle_deg + 360.0) % 360.0;

        let direction = match normalized_degree {
            x if !(22.5..337.5).contains(&x) => Direction::Down,
            x if (22.5..67.5).contains(&x) => Direction::BottomRight,
            x if (67.5..112.5).contains(&x) => Direction::Right,
            x if (112.5..157.5).contains(&x) => Direction::TopRight,
            x if (157.5..202.5).contains(&x) => Direction::Up,
            x if (202.5..247.5).contains(&x) => Direction::TopLeft,
            x if (247.5..292.5).contains(&x) => Direction::Left,
            _ => Direction::BottomLeft,
        };

        (direction, force)
    }
}
