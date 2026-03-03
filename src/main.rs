#![no_std]
#![no_main]

mod max7219;

use arduino_hal::Spi;
use arduino_hal::default_serial;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

use avr_servo::{ServoPin, ServoPinOps};
use max7219::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // let mut spi = Spi::new(dp.SPI, pins.d52.into_output(), pins.d51.into_output(), pins.d50.into_pull_up_input(), pins.d53.into_output(), arduino_hal::spi::Settings::default());

    let mut data_pin = pins.d3.into_output().downgrade();
    let mut cs_pin = pins.d4.into_output().downgrade();
    let mut clk_pin = pins.d5.into_output().downgrade();
    // let tc = dp.TC3;
    // let servo = ServoPin::new(&tc, pins.d5);

    // Joystick min = 0
    // Joystick max = 1023
    send_data(&mut data_pin, &mut clk_pin, &mut cs_pin, 0b00001111, Address::DIGIT0);

    loop {
    }
}
