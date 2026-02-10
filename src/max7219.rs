//! Starting point for a driver for the MAX7219 8x8 LED matrix.
#![allow(dead_code)]

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


// Connectivity:
// * MOSI to DIN
// * I/O to LOAD(CS)
// * SCK to CLK

// SPI data
// * CLK period min = 100 ns
// * CLK pulse width high min = 50 ns