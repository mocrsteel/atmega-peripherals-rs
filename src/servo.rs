//! # Servo
//!
//! Servo interface for the Arduino Mega 2560 board.
//!
//! ## Servo control
//!
//! * 20 ms PWM period required = 50 Hz
//! * Control range: 0.5 to 2.5 ms for a 0 to 180 degrees rotation.
//!
//! ## Arduino Mega2560 interface
//!
//! * Timer/Counters
//!   * TC0: 8-bit
//!   * TC1: 16-bit
//!   * TC2: 8-bit
//!   * TC3: 16-bit
//!   * TC4: 16-bit
//!   * TC5: 16-bit
//!
//! This driver requires the board to have 16-bit capable timer/counter registers.
//!
//! ### 16-bit Timer/Counter application
//!
//! #### Prescaler selection
//!
//! Best possible control resolution (Direct clock, no prescaler):
//! * 16 bit register = 2^16 = 65 536 maximum range for TOP.
//! * Prescaler:
//!     * Direct: 16 MHz = 62.5 ns per clock tick --> 4 ms maximum cycle time.
//!     * Prescale8: 16M/8 = 2 MHz = 0.5 us per clock tick
//!         * 32.8 ms maximum cycle time
//!         * Resolution: 180 degrees in 4000 steps = 0.045 degrees per step.
//!     * Prescale64: 16M/64 = 250 kHz = 4 us per clock tick
//!         * 262 ms maximum cycle time
//!         * Resolution: 180 degrees in  500 steps = 0.36 degrees per tick.
//!
//! #### PWM configuration
//!
//! We want to achieve a 50 Hz PWM period. We'll use WGM mode 14 as with this mode
//! we're able to set TOP using ICRn. That will define the overall PWM frequency.
//! The duty will be set using the channel output compare OCRnx.
//!
//! The desired frequency calculation is defined in ATMega docs on page 148:
//!
//! f<sub>OCnxPWM</sub> = f<sub>csl_I/O</sub> / (N * (1 + TOP))
//!
//! where:
//! * f<sub>clk_I/O</sub> is the system clock frequency in Hz (16e6 Hz).
//! * f<sub>OCnxPWM</sub> is the PWM frequency in Hz.
//! * N is the prescale factor (1, 8, 64, 256 or 1024).
//!
//! ##### Prescale8
//!
//! To achieve a 50 Hz PWM frequency with Prescale8, we need to set TOP to 39 999 (0xC34F).
//!
//! * 0.5 ms = 1000 ticks (0 - 999)
//! * 2.0 ms = 4000 ticks (0 - 3999)
//! * 2.5 ms = 5000 ticks (0 - 4999)
//!
//! ##### Prescale64
//!
//! To achieve a 50 Hz PWM frequency with Prescale64, we need to set TOP to 4999 (0x1387).
//!
//! We'll choose for Fast PWM with TOP set by ICRx (1, 3, 4 or 5).
//! OCRnx will be used to set the duty cyle.
//!
//! Control range with prescale64:
//! * 0.5 ms = 125 ticks
//! * 2.0 ms = 500 ticks (0 -> 499)
//! * 2.5 ms = 625 ticks

use arduino_hal::hal::port::{PB4, PB5, PB6};
use arduino_hal::pac::{TC1, TC3, TC4, TC5};
use arduino_hal::port::{
    mode::{Floating, Input, Output},
    Pin,
};

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
/// Output compare mode for FastPWM: Clear OCnx on compare match, set OCnA at BOTTOM. Register TCCRB COMnA/B/C 0:1
pub static COM1A: u8 = 0b10;

/// Trait implements standard functions required to operate a servo for a given Timer/Counter `TC`
/// and a pin `PIN`.
pub trait ServoPinOps<'a, TC, PIN> {
    /// Takes ownership of a pin and uses the associated Timer/Counter to set up the pin for servo
    /// operations. The created Servo instance contains all functions required.
    fn new(tc: &'a TC, pin: PIN) -> Self;
    /// **For testing purpose. Delete later.**
    ///
    /// Set the PWM duty manually. Theoretical control range for a servo:
    /// * 0.5 ms to 2.5 ms for 0 degrees to 180 degrees.
    /// * With Prescale8, matches 999 to 4999.
    /// * With Prescale64, matches 124 to 624.
    fn set_duty(&self, duty: u16);
    /// Change the servo's mode of operation to [ServoMode::Fast] or [ServoMode::Precise].
    /// > **Warning:** This operation applies to the `TCx` in general and will affect all pins using
    /// TC1!
    fn set_mode(&mut self, mode: ServoMode);
    /// Get the current mode in form of a `&str`.
    fn get_mode(&self) -> &str;
    /// Set the rotation in degrees.
    fn set_rotation(&self, degrees: u8) -> u16;
}

/// Takes a pin and a reference to a timer/counter for the servo driver.
/// Restricts the combinations of pin - TC as per the ATMega2560 documentation.
///
/// > Note: Built for the Elegoo ATMega2560. Compatible with the Arduino Mega board.
///
/// The servo will always be initalized at 0 degrees of rotation.
pub struct ServoPin<'a, TC, PIN> {
    /// Pin that will control the servo PWM channel.
    pin: PIN,
    /// Timer/Counter. Can be TC1, 3, 4 or 5 (16-bit).
    tc: &'a TC,
    /// Servo mode of operation. Defaults to [ServoMode::Fast]
    mode: ServoMode,
}

impl<'a> ServoPinOps<'a, TC1, Pin<Input<Floating>, PB5>> for ServoPin<'a, TC1, Pin<Output, PB5>> {
    /// Servo pin operations generator for:
    /// * Port PB5
    /// * Pin D11
    /// * Timer/Counter 1
    /// * Output Compare Channel A
    ///
    /// Initially sets up to [ServoMode::Fast]. Can be set to [ServoMode::Precise] by using method
    /// `ServoPin.set_mode()`
    fn new(tc: &'a TC1, pin: Pin<Input<Floating>, PB5>) -> Self {
        tc.tccr1a().reset();
        tc.tccr1b().reset();
        tc.tccr1c().reset();

        // tc.tccr1a().write(|w| w.com1a().set(COM1A).wgm1().set(WGM01));
        // tc.tccr1b().write(|w| w.wgm1().set(WGM23).cs1().prescale_64());
        tc.tccr1a()
            .write(|w| w.com1a().set(COM1A).wgm1().set(WGM01));
        tc.tccr1b()
            .write(|w| w.wgm1().set(WGM23).cs1().prescale_64());

        tc.icr1().write(|w| w.set(PRESCALE64_PWM_TOP));
        tc.ocr1a().write(|w| w.set(PRESCALE64_SERVO_MIN));

        let pin = pin.into_output();

        ServoPin {
            pin,
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
        self.tc.tccr1b().modify(|_, w| unsafe { w.cs1().bits(cs) });
        self.tc.icr1().write(|w| w.set(top));
        self.tc.ocr1a().write(|w| w.set(min));
    }

    fn set_duty(&self, duty: u16) {
        self.tc.ocr1a().write(|w| w.set(duty));
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

        self.tc.ocr1a().write(|w| w.set(setpoint));

        setpoint
    }
}
