//! Base code for a servo driver implementations.
//! Can be applied to most AVR based chipsets.
//! 
//! # Applications
//! 
//! Built and tested for:
//! 
//! * `ATMega2560` (Arduino Mega)
//! 
//! # Example
//! ```
//! use arduino_hal::hal::port::{PB7};
//! use arduino_hal::pac::{TC1};
//! use crate::avr_servo::*;
//! use crate::impl_servo;
//! 
//! impl_servo! {
//!         /// * Port: PB7
//!         /// * Timer/Counter1 (16-bit)
//!         /// * Output compare channel C
//!         /// * Pin D13
//!         pin: PB7,
//!         tc: 1,
//!         channel: c,
//! }
//! ```

pub use core::marker::PhantomData;
pub use arduino_hal::port::{
    mode::{Floating, Input},
    Pin,
};
pub use paste::paste;

/// Modes of operation available for a servo.
///
/// ## Details
///
/// Fast mode uses Prescale64, leading to a slower clock rate (250 kHz).
/// Precise mode uses Prescale8, leading to a faster clock rate (2 MHz).
///
/// With a faster clock, there are more 'clock ticks' within the 2 ms range of control for a typical
/// servo motor. This means you can achieve a preciser control on the servo. If you want to iterate
/// over the full 180 degrees, skipping steps will be required since more clock ticks need to be
/// passed to increase rotation by 1 degree.
#[derive(Clone, Copy)]
pub enum ServoMode {
    /// Allows for faster movement but at the cost of precision (0.36 degrees resolution).
    Fast,
    /// Allows for more precise movement but at the cost of speed (0.045 degrees resolution).
    Precise,
}

/// Clock select Prescale8
pub static CS_8: u8 = 0b010;
/// Clock select Prescale64
pub static CS_64: u8 = 0b011;
/// Counter value at 0.5 ms for Prescale8
pub static PRESCALE8_SERVO_MIN: u16 = 999;
/// Counter value at 2.5 ms for Prescale8
pub static PRESCALE8_SERVO_MAX: u16 = 4999;
/// Counter value at 20 ms for Prescale8
pub static PRESCALE8_PWM_TOP: u16 = 39999;

/// Counter value at 0.5 ms for Prescale64
pub static PRESCALE64_SERVO_MIN: u16 = 124;
/// Counter value at 2.5 ms for Prescale64
pub static PRESCALE64_SERVO_MAX: u16 = 624;
/// Counter value at 20 ms for Prescale64
pub static PRESCALE64_PWM_TOP: u16 = 4999;

/// FastPWM (mode 14) 0b1110 split in WGM 0:1 (TCCRxA) and 2:3 (TCCRxB)
pub static WGM01: u8 = 0b10;
/// FastPWM (mode 14) 0b1110 split in WGM 0:1 (TCCRxA) and 2:3 (TCCRxB)
pub static WGM23: u8 = 0b11;
/// Output compare mode for FastPWM: Clear OCnx on compare match, set OCnA at BOTTOM. Register TCCRxB COMnA/B/C 0:1
pub static COM1A: u8 = 0b10;

/// Trait implements standard functions required to operate a servo for a given Timer/Counter `TC`
/// and a pin `PIN`.
pub trait ServoPinOps<'a, TC, PIN> {
    /// Takes ownership of a pin and uses the associated Timer/Counter to set up the pin for servo
    /// operations. The created Servo instance contains all functions required.
    fn new(tc: &'a TC, pin: PIN) -> Self;
    /// Change the servo's mode of operation to [crate::ServoMode::Fast] or [crate::ServoMode::Precise].
    /// > **Warning:** This operation applies to the `TCx` in general and will affect all pins using
    /// TC1!
    fn set_mode(&mut self, mode: ServoMode);
    /// Get the current mode in form of a `&str`.
    fn get_mode(&self) -> &str;
    // /// **For testing purpose. Delete later.**
    // ///
    // /// Set the PWM duty manually. Theoretical control range for a servo:
    // /// * 0.5 ms to 2.5 ms for 0 degrees to 180 degrees.
    // /// * With Prescale8, matches 999 to 4999.
    // /// * With Prescale64, matches 124 to 624.
    // fn set_duty(&self, duty: u16);
    /// Set the rotation in degrees.
    /// Range is from 0 to 180 degrees.
    fn set_rotation(&self, degrees: u8) -> u16;
}

/// Takes a pin and a reference to a timer/counter for the servo driver.
/// Restricts the combinations of pin - TC as per the ATMega2560 documentation.
///
/// > Note: Built for the Elegoo ATMega2560. Compatible with the Arduino Mega board.
///
/// The servo will always be initialized at 0 degrees of rotation.
pub struct ServoPin<'a, TC, PIN> {
    /// Pin that will control the servo PWM channel.
    pub(crate)pin: PhantomData<PIN>,
    /// Timer/Counter. Can be TC1, 3, 4 or 5 (16-bit).
    pub(crate)tc: &'a TC,
    /// Servo mode of operation. Defaults to [ServoMode::Fast]
    pub(crate)mode: ServoMode,
}

/// Servo to implement the [ServoPinOps] trait for the different pins defines.
/// 
/// # How to use
/// 
/// Use this macro to implement the ServoPin and ServoPinOps for the required AVR board.
/// 
/// Fields to be filled in the macro:
/// * `pin`: The `arduino-hal` defined port for your MCU.
/// * `tc`: The Timer/Counter identifier. Must be a 16-bit compatible timer/counter.
/// * `channel`: The correct output compare channel for the port (i.e. in the example: port `PB7` uses `OC1C`).
/// 
/// ```
/// use arduino_hal::hal::port::PB7;
/// use arduino_hal::pac::TC1;
/// use crate::avr_servo::*;
/// use crate::impl_servo;
/// 
/// impl_servo! {
///         /// * Port: PB7
///         /// * Timer/Counter1 (16-bit)
///         /// * Output compare channel C
///         /// * Pin D13
///         pin: PB7,
///         tc: 1,
///         channel: c,
/// }
/// ```
#[macro_export]
macro_rules! impl_servo {
    (
        $(#[$server_pwm_doc:meta])*
        pin: $pin:ty,
        tc: $tc:literal,
        channel: $ch:ident,
    ) => {
        paste! {
            $(#[$server_pwm_doc])*
            impl<'a> ServoPinOps<'a, [<TC $tc>], Pin<Input<Floating>, $pin>> for ServoPin<'a, [<TC $tc>], Pin<Input<Floating>, $pin>> {
                fn new(tc: &'a [<TC $tc>], pin: Pin<Input<Floating>, $pin>) -> Self {
                    tc.[<tccr $tc a>]().reset();
                    tc.[<tccr $tc b>]().reset();
                    tc.[<tccr $tc c>]().reset();

                    tc.[<tccr $tc a>]().write(|w| w.[<com $tc $ch>]().set(COM1A).[<wgm $tc>]().set(WGM01));
                    tc.[<tccr $tc b>]().write(|w| w.[<wgm $tc>]().set(WGM23).[<cs $tc>]().prescale_64());

                    tc.[<icr $tc>]().write(|w| w.set(PRESCALE64_PWM_TOP));
                    tc.[<ocr $tc $ch>]().write(|w| w.set(PRESCALE64_SERVO_MIN));

                    pin.into_output();

                    ServoPin {
                        pin: PhantomData,
                        tc,
                        mode: ServoMode::Fast,
                    }
                }

                fn set_mode(&mut self, mode: ServoMode) {
                    self.mode = mode;
                    let (top, min, cs) = match mode {
                        ServoMode::Fast => (PRESCALE64_PWM_TOP, PRESCALE64_SERVO_MIN, CS_64),
                        ServoMode::Precise => (PRESCALE8_PWM_TOP, PRESCALE8_SERVO_MIN, CS_8),
                    };
                    self.tc.[<tccr $tc b>]().modify(|_, w| unsafe { w.[<cs $tc>]().bits(cs) } );
                    self.tc.[<icr $tc>]().write(|w| w.set(top));
                    self.tc.[<ocr $tc $ch>]().write(|w| w.set(min));
                }

                fn get_mode(&self) -> &str {
                    match self.mode {
                        ServoMode::Fast => "ServoMode::Fast",
                        ServoMode::Precise => "ServoMode::Precise",
                    }
                }

                fn set_rotation(&self, degrees: u8) -> u16 {
                    let (pwm_min, pwm_max) = match self.mode {
                        ServoMode::Fast => (PRESCALE64_SERVO_MIN as u32, PRESCALE64_SERVO_MAX as u32),
                        ServoMode::Precise => (PRESCALE8_SERVO_MIN as u32, PRESCALE8_SERVO_MAX as u32),
                    };
                    let range = pwm_max - pwm_min;
                    let setpoint = (pwm_min + (range * degrees as u32 * 10) / (180 * 10)) as u16;
                    self.tc.[<ocr $tc $ch>]().write(|w| w.set(setpoint));

                    setpoint
                }
            }
        }
    }
}
