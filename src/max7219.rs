//! Starting point for a driver for the MAX7219 8x8 LED matrix.
#![allow(dead_code)]

use arduino_hal::port::Pin;
use arduino_hal::hal::port::{Dynamic, mode};

// -- Register addresses per digit (rows?)
const DIG_0: u8 = 0x0;
const DIG_1: u8 = 0x1;
const DIG_2: u8 = 0x2;
const DIG_3: u8 = 0x3;
const DIG_4: u8 = 0x4;
const DIG_5: u8 = 0x5;
const DIG_6: u8 = 0x6;
const DIG_7: u8 = 0x7;
const NO_OP: u8 = 0x0;
const MODE_DECODE: u8 = 0x9;
const INTENSITY: u8 = 0xA;
const SCAN_LIMIT: u8 = 0xB;
const SHUTDOWN: u8 = 0xC;
const DISPLAY_TEST: u8 = 0xF;

// Intensity modes
const INTENSITY_MIN: u8 = 0x0;
const INTENSITY_MAX: u8 = 0xF;

pub enum ADDRESS {
    DIG_0,
    DIG_1,
    DIG_2,
    DIG_3,
    DIG_4,
    DIG_5,
    DIG_6,
    DIG_7,
    NO_OP,
    MODE_DECODE,
    INTENSITY,
    SCAN_LIMIT,
    SHUTDOWN,
    DISPLAY_TEST,
}
// Connectivity:
// * MOSI to DIN
// * I/O to LOAD(CS)
// * SCK to CLK

// SPI data
// * CLK period min = 100 ns
// * CLK pulse width high min = 50 ns

// Serial data format:
// D15 - D12 : Not used
// D11 - D8: Address
// D7 - D0: MSB to LSB of data.
// MAx7219 needs to receive the MSB first.

/// Manual bit bang code.
pub fn send_data<'a>(pin: &'a mut Pin<mode::Output,>, clk: &'a mut Pin<mode::Output,>, data: u8, address: u8) {
    // Combine the address to the first 8 bits and then append the data in the last 8 bits.
    let serialized = (address as u16) << 8 | (data as u16); 

    clk.set_low();

}