//! Starting point for a driver for the MAX7219 8x8 LED matrix.
#![allow(dead_code)]

use arduino_hal::hal::port::{mode::Output, Dynamic};
use arduino_hal::port::Pin;

/// Possible register addresses for the MAX7219.
pub struct Address;
impl Address {
    pub const DIGIT0: u8 = 0x1;
    pub const DIGIT1: u8 = 0x2;
    pub const DIGIT2: u8 = 0x3;
    pub const DIGIT3: u8 = 0x4;
    pub const DIGIT4: u8 = 0x5;
    pub const DIGIT5: u8 = 0x6;
    pub const DIGIT6: u8 = 0x7;
    pub const DIGIT7: u8 = 0x8;
    pub const NO_OP: u8 = 0x0;
    pub const DECODE_MODE: u8 = 0x9;
    pub const INTENSITY: u8 = 0xa;
    pub const SCAN_LIMIT: u8 = 0xb;
    pub const SHUTDOWN: u8 = 0xc;
    pub const DISPLAY_TEST: u8 = 0xf;
}

/// Intensity for MAX7219. Can be set to MAX `0xf` or MIN `0x0`.
/// Can be set to intermediate values using [Intensity::val()]
pub struct Intensity;
impl Intensity {
    pub const MIN: u8 = 0x0;
    pub const MAX: u8 = 0xf;

    /// Get a validated intensity value for the MAX7219.
    /// Ensures that the value is between MIN and MAX (0x0 and 0xf).
    pub fn val(intensity: u8) -> u8 {
        if intensity < Self::MIN {
            Self::MIN
        } else if intensity > Self::MAX {
            Self::MAX
        } else {
            intensity
        }
    }
}

// Connectivity:
// * clk pin to CLK
// * cs pin to CS
// * data pin to DIN
// * 5V to VCC
// * GND to GND

// SPI data
// * CLK period min = 100 ns
// * CLK pulse width high min = 50 ns

// Serial data format:
// D15 - D12 : Not used
// D11 - D8: Address
// D7 - D0: MSB to LSB of data.
// MAx7219 needs to receive the MSB first.

/// Manually send data to the MAX7219 LED matrix.
pub fn send_data<'a>(
    data_pin: &'a mut Pin<Output, Dynamic>,
    clk_pin: &'a mut Pin<Output, Dynamic>,
    cs_pin: &'a mut Pin<Output, Dynamic>,
    data: u8,
    address: u8,
) {
    // Combine the address to the first 8 bits and then append the data in the last 8 bits.
    let mut serialized = (address as u16) << 8 | (data as u16);
    cs_pin.set_high();
    arduino_hal::delay_ns(50);
    cs_pin.set_low();
    // prepare to receive data.
    clk_pin.set_low();
    arduino_hal::delay_ns(50);

    // Serially shift the 16 data bits into shift register (address + data).
    for _ in 0..16 {
        match (serialized >> 15) & 1 == 1 {
            false => data_pin.set_low(),
            true => data_pin.set_high(),
        };
        serialized <<= 1;
        clk_pin.set_high();
        arduino_hal::delay_ns(50);
        clk_pin.set_low();
        arduino_hal::delay_ns(50);
    }
    // pulse to load to register address.
    cs_pin.set_high();
    arduino_hal::delay_ns(50);
    cs_pin.set_low();
}

pub fn set_matrix<'a>(
    data_pin: &'a mut Pin<Output, Dynamic>,
    clk_pin: &'a mut Pin<Output, Dynamic>,
    cs_pin: &'a mut Pin<Output, Dynamic>,
    data: [u8; 8]
) {
    // Initialize the LED matrix
    // TODO: This needs to be ported into a matrix struct to only initialize once!

    // ...
    
    // Digit 0 = 0x1, Digit 7 - 0x8.
    for i in 1..=8u8 {
        let address = i;
        let byte = data[(i as usize) - 1];
        send_data(data_pin, clk_pin, cs_pin, byte, address);
    }
}
