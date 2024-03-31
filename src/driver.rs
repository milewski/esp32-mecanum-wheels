use std::ops::{Deref, DerefMut};

use esp_idf_hal::gpio::{AnyOutputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::sys::EspError;

pub trait MotorDriver {
    fn forward(&mut self) -> Result<(), EspError>;
    fn reverse(&mut self) -> Result<(), EspError>;
    // fn set_low(&mut self) -> Result<(), EspError>;
    // fn set_high(&mut self) -> Result<(), EspError>;

    /// Sets the motor driver to coast mode, allowing the motor to freely spin or coast to a stop
    /// without applying any active driving or braking force. In this mode, both the forward and
    /// reverse inputs are set low, disconnecting the motor from the driver circuitry. This allows
    /// the motor to naturally decelerate and come to a stop based on its inertia or external forces.
    /// Coast mode is useful when a smooth and natural deceleration of the motor is desired, such as
    /// when transitioning between motor states or when manual control requires the motor to spin
    /// freely without any active driving or braking.
    fn coast(&mut self) -> Result<(), EspError>;

    /// Sets the motor driver to stop mode, causing the motor to rapidly come to a halt by
    /// applying a fast decay to the current in the motor winding. In fast decay, the magnetic field
    /// around the motor winding collapses quickly when the motor driver switches off, resulting in
    /// rapid deceleration. This mode is beneficial for achieving fast motor response times and
    /// transitioning between motor states quickly. However, it may produce higher levels of
    /// electrical noise due to the rapid changes in current. Use stop mode when immediate stopping
    /// of the motor is required, accepting the trade-off of potential electrical noise.
    fn stop(&mut self) -> Result<(), EspError>;
}

pub struct Bridge<'d, IN1, IN2>
    where
        IN1: OutputPin,
        IN2: OutputPin,
{
    in1: PinDriver<'d, IN1, Output>,
    in2: PinDriver<'d, IN2, Output>,
}

impl<'d, IN1, IN2> Bridge<'d, IN1, IN2>
    where
        IN1: OutputPin,
        IN2: OutputPin,
{
    pub fn new(
        in1: PinDriver<'d, IN1, Output>,
        in2: PinDriver<'d, IN2, Output>,
    ) -> Self {
        Self { in1, in2 }
    }

    pub fn forward(&mut self) -> Result<(), EspError> {
        self.in1.set_high()?;
        self.in2.set_low()?;

        Ok(())
    }

    pub fn reverse(&mut self) -> Result<(), EspError> {
        self.in1.set_low()?;
        self.in2.set_high()?;

        Ok(())
    }

    pub fn coast(&mut self) -> Result<(), EspError> {
        self.in1.set_low()?;
        self.in2.set_low()?;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), EspError> {
        self.in1.set_high()?;
        self.in2.set_high()?;

        Ok(())
    }
}

pub struct SyncDriver<'d, IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pub a: Bridge<'d, IN1, IN2>,
    pub b: Bridge<'d, IN3, IN4>,
}

impl<'d, IN1: OutputPin, IN2: OutputPin, IN3: OutputPin, IN4: OutputPin> SyncDriver<'d, IN1, IN2, IN3, IN4> {
    pub fn new(
        in1: PinDriver<'d, IN1, Output>,
        in2: PinDriver<'d, IN2, Output>,
        in3: PinDriver<'d, IN3, Output>,
        in4: PinDriver<'d, IN4, Output>,
    ) -> Self {
        Self {
            a: Bridge::new(in1, in2),
            b: Bridge::new(in3, in4),
        }
    }
}

// impl<'d, IN1, IN2, IN3, IN4> Deref for SyncDriver<'d, IN1, IN2, IN3, IN4> {
//     type Target = Bridge<'d, IN1, IN2>;
//
//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }
//
// impl<'d, IN1, IN2, IN3, IN4> DerefMut for SyncDriver<'d, IN1, IN2, IN3, IN4> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self
//     }
// }

impl<'d, IN1, IN2, IN3, IN4> MotorDriver for SyncDriver<'d, IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    fn forward(&mut self) -> Result<(), EspError> {
        todo!()
    }

    fn reverse(&mut self) -> Result<(), EspError> {
        todo!()
    }

    fn coast(&mut self) -> Result<(), EspError> {
        todo!()
    }

    fn stop(&mut self) -> Result<(), EspError> {
        todo!()
    }
}

pub struct ParallelDriver<'d, IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    a: Bridge<'d, IN1, IN2>,
    b: Bridge<'d, IN3, IN4>,
}

impl<'d, IN1, IN2, IN3, IN4> MotorDriver for ParallelDriver<'d, IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    fn forward(&mut self) -> Result<(), EspError> {
        self.a.forward()?;
        self.b.forward()?;

        Ok(())
    }

    fn reverse(&mut self) -> Result<(), EspError> {
        self.a.reverse()?;
        self.b.reverse()?;

        Ok(())
    }

    fn coast(&mut self) -> Result<(), EspError> {
        self.a.coast()?;
        self.b.coast()?;

        Ok(())
    }

    fn stop(&mut self) -> Result<(), EspError> {
        self.a.stop()?;
        self.b.stop()?;

        Ok(())
    }
}

impl<'d, IN1, IN2, IN3, IN4> ParallelDriver<'d, IN1, IN2, IN3, IN4>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        IN3: OutputPin,
        IN4: OutputPin,
{
    pub fn new(
        in1: PinDriver<'d, IN1, Output>,
        in2: PinDriver<'d, IN2, Output>,
        in3: PinDriver<'d, IN3, Output>,
        in4: PinDriver<'d, IN4, Output>,
    ) -> Self {
        Self {
            a: Bridge::new(in1, in2),
            b: Bridge::new(in3, in4),
        }
    }
}

pub struct DRV8833Driver<'d, DRIVER, SLEEP>
    where
        DRIVER: MotorDriver,
        SLEEP: OutputPin,
{
    driver: DRIVER,
    sleep: PinDriver<'d, SLEEP, Output>,
}

impl<'d, SLEEP: OutputPin> DRV8833Driver<'d, SyncDriver<'d, AnyOutputPin, AnyOutputPin, AnyOutputPin, AnyOutputPin>, SLEEP> {
    pub fn new_sync<A, B, C, D>(
        in1: PinDriver<'d, A, Output>,
        in2: PinDriver<'d, B, Output>,
        in3: PinDriver<'d, C, Output>,
        in4: PinDriver<'d, D, Output>,
        sleep: PinDriver<'d, SLEEP, Output>,
    ) -> DRV8833Driver<'d, SyncDriver<'d, A, B, C, D>, SLEEP>
        where
            A: OutputPin,
            B: OutputPin,
            C: OutputPin,
            D: OutputPin,
    {
        DRV8833Driver {
            driver: SyncDriver::new(in1, in2, in3, in4),
            sleep,
        }
    }
}

impl<'d, SLEEP: OutputPin> DRV8833Driver<'d, ParallelDriver<'d, AnyOutputPin, AnyOutputPin, AnyOutputPin, AnyOutputPin>, SLEEP> {
    /// Creates a new motor driver instance configured for parallel mode, where both bridges are
    /// controlled identically. This mode is useful for two main purposes:
    /// 1. Increasing current output: By connecting IN1 with IN3 and IN2 with IN4, you can effectively
    ///    double the current output capability, as both bridges operate together to drive the motor.
    /// 2. Ensuring identical behavior: If you need both bridges to behave exactly the same, parallel
    ///    mode ensures synchronous control of the motor.
    pub fn new_parallel<A, B, C, D>(
        in1: PinDriver<'d, A, Output>,
        in2: PinDriver<'d, B, Output>,
        in3: PinDriver<'d, C, Output>,
        in4: PinDriver<'d, D, Output>,
        sleep: PinDriver<'d, SLEEP, Output>,
    ) -> DRV8833Driver<'d, ParallelDriver<'d, A, B, C, D>, SLEEP>
        where
            A: OutputPin,
            B: OutputPin,
            C: OutputPin,
            D: OutputPin,
    {
        DRV8833Driver {
            driver: ParallelDriver::new(in1, in2, in3, in4),
            sleep,
        }
    }
}

impl<'d, DRIVER: MotorDriver, SLEEP: OutputPin> Deref for DRV8833Driver<'d, DRIVER, SLEEP> {
    type Target = DRIVER;

    fn deref(&self) -> &Self::Target {
        &self.driver
    }
}

impl<'d, DRIVER: MotorDriver, SLEEP: OutputPin> DerefMut for DRV8833Driver<'d, DRIVER, SLEEP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.driver
    }
}

impl<'d, DRIVER, SLEEP> DRV8833Driver<'d, DRIVER, SLEEP>
    where
        DRIVER: MotorDriver,
        SLEEP: OutputPin,
{
    pub fn sleep(&mut self) -> Result<(), EspError> {
        self.sleep.set_low()
    }

    pub fn wakeup(&mut self) -> Result<(), EspError> {
        self.sleep.set_high()
    }

    pub fn is_fault(&self) -> bool {
        true
    }
}

trait MotorDriverTraitTemp {
    fn sleep(&mut self) -> Result<(), EspError>;
    fn wakeup(&mut self) -> Result<(), EspError>;
    fn stop(&mut self) -> Result<(), EspError>;
    fn coast(&mut self) -> Result<(), EspError>;
    fn forward(&mut self) -> Result<(), EspError>;
    fn reverse(&mut self) -> Result<(), EspError>;
}
