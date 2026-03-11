#![no_std]
#![no_main]

mod max7219;

use arduino_hal::default_serial;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

// use avr_servo::{ServoPin, ServoPinOps};
use max7219::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);
    // let mut spi = Spi::new(dp.SPI, pins.d52.into_output(), pins.d51.into_output(), pins.d50.into_pull_up_input(), pins.d53.into_output(), arduino_hal::spi::Settings::default());

    let mut data_pin = pins.d3.into_output().downgrade();
    let mut cs_pin = pins.d4.into_output().downgrade();
    let mut clk_pin = pins.d5.into_output().downgrade();

    let address: u8 = 0xF;
    let data: u8 = 0x1;

    send_data(&mut data_pin, &mut clk_pin, &mut cs_pin, 0x1u8, Address::DISPLAY_TEST);
    arduino_hal::delay_ms(1000);
    send_data(&mut data_pin, &mut clk_pin, &mut cs_pin, 0x0u8, Address::DISPLAY_TEST);
    send_data(&mut data_pin, &mut clk_pin, &mut cs_pin, 0x0u8, Address::DECODE_MODE);
    send_data(&mut data_pin, &mut clk_pin, &mut cs_pin, 0x7u8, Address::SCAN_LIMIT);
    let flag_down: [u8; 8] = [
        0b00000000,
        0b00000001,
        0b00000011,
        0b00000111,
        0b00001111,
        0b00011111,
        0b00111111,
        0b01111111,
    ];
    let flag_up: [u8; 8] = [
        0b11111110,
        0b11111100,
        0b11111000,
        0b11110000,
        0b11100000,
        0b11000000,
        0b10000000,
        0b00000000,
    ];
    
    send_data(&mut data_pin, &mut clk_pin, &mut cs_pin, 0x1u8, Address::INTENSITY);
    
    loop {
        set_matrix(&mut data_pin, &mut clk_pin, &mut cs_pin, flag_up);
        arduino_hal::delay_ms(500);
        set_matrix(&mut data_pin, &mut clk_pin, &mut cs_pin, flag_down);
        arduino_hal::delay_ms(500);

    }
}
