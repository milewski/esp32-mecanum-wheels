use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

use drv8833_driver::{Breaks, MotorDriverError, Movement, PwmSplitSingleDriverType};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::pwm::SetDutyCycle;

pub struct Mecanum<MotorA, MotorB> {
    motor_a: MotorA,
    motor_b: MotorB,
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

impl<IN1, IN2, IN3, IN4, PWM, FAULTA, IN1B, IN2B, IN3B, IN4B, PWMB, FAULTB> Mecanum<
    PwmSplitSingleDriverType<IN1, IN2, IN3, IN4, PWM, FAULTA>,
    PwmSplitSingleDriverType<IN1B, IN2B, IN3B, IN4B, PWMB, FAULTB>
>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
        IN1B: OutputPin,
        IN2B: OutputPin,
        IN3B: OutputPin,
        IN4B: OutputPin,
        PWM: SetDutyCycle,
        PWMB: SetDutyCycle,
        FAULTA: InputPin,
        FAULTB: InputPin
{
    pub fn new(
        motor_a: PwmSplitSingleDriverType<IN1, IN2, IN3, IN4, PWM, FAULTA>,
        motor_b: PwmSplitSingleDriverType<IN1B, IN2B, IN3B, IN4B, PWMB, FAULTB>,
    ) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { motor_a, motor_b }))
    }

    fn set_duty_cycle(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.motor_a.set_duty_cycle(force)?;
        self.motor_b.set_duty_cycle(force)?;

        Ok(())
    }

    pub fn spin(&mut self, x: u8, y: u8) -> Result<(), MotorDriverError> {
        let (direction, force) = self.interpret_direction_and_force(x, y);

        match force {
            0 => self.stop()?,
            _ => match direction {
                Direction::Left => self.spin_left(force)?,
                Direction::Right => self.spin_right(force)?,
                _ => unreachable!()
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(0)?;

        self.motor_a.a.coast()?;
        self.motor_b.b.coast()?;

        Ok(())
    }

    pub fn update(&mut self, x: u8, y: u8) -> Result<(), MotorDriverError> {
        let (direction, force) = self.interpret_direction_and_force(x, y);

        match force {
            0 => self.stop()?,
            _ => match direction {
                Direction::Up => self.up(force)?,
                Direction::Down => self.down(force)?,
                Direction::Left => self.left(force)?,
                Direction::Right => self.right(force)?,
                Direction::TopRight => self.top_right(force)?,
                Direction::TopLeft => self.top_left(force)?,
                Direction::BottomLeft => self.bottom_left(force)?,
                Direction::BottomRight => self.bottom_right(force)?,
            }
        }

        Ok(())
    }

    pub fn right(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.reverse()?;
        self.motor_a.b.forward()?;

        self.motor_b.a.forward()?;
        self.motor_b.b.reverse()?;

        Ok(())
    }

    pub fn left(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.forward()?;
        self.motor_a.b.reverse()?;

        self.motor_b.a.reverse()?;
        self.motor_b.b.forward()?;

        Ok(())
    }

    pub fn up(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.reverse()?;
        self.motor_a.b.reverse()?;

        self.motor_b.a.reverse()?;
        self.motor_b.b.reverse()?;

        Ok(())
    }

    pub fn down(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.forward()?;
        self.motor_a.b.forward()?;

        self.motor_b.a.forward()?;
        self.motor_b.b.forward()?;

        Ok(())
    }

    pub fn top_right(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.reverse()?;
        self.motor_a.b.coast()?;

        self.motor_b.a.coast()?;
        self.motor_b.b.reverse()?;

        Ok(())
    }

    pub fn top_left(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.coast()?;
        self.motor_a.b.reverse()?;

        self.motor_b.a.reverse()?;
        self.motor_b.b.coast()?;

        Ok(())
    }

    pub fn bottom_left(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.forward()?;
        self.motor_a.b.coast()?;

        self.motor_b.a.coast()?;
        self.motor_b.b.forward()?;

        Ok(())
    }

    pub fn bottom_right(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.coast()?;
        self.motor_a.b.forward()?;

        self.motor_b.a.forward()?;
        self.motor_b.b.coast()?;

        Ok(())
    }

    pub fn spin_right(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.reverse()?;
        self.motor_a.b.forward()?;

        self.motor_b.a.reverse()?;
        self.motor_b.b.forward()?;

        Ok(())
    }

    pub fn spin_left(&mut self, force: u8) -> Result<(), MotorDriverError> {
        self.set_duty_cycle(force)?;

        self.motor_a.a.forward()?;
        self.motor_a.b.reverse()?;

        self.motor_b.a.forward()?;
        self.motor_b.b.reverse()?;

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
