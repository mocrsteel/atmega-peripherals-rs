#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::{default_serial};
use panic_halt as _;
use ufmt::uwriteln;

use avr_servo::{ServoPin, ServoPinOps};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);

    let tc = dp.TC3;
    let servo = ServoPin::new(&tc, pins.d5);
    
    // Joystick min = 0
    // Joystick max = 1023
   
    loop {
        for deg in [0, 45, 90, 135, 180, 135, 90, 45] {
            servo.set_rotation(deg as u8);
            uwriteln!(&mut serial, "Rotating to {} degrees", deg).unwrap_infallible();
            arduino_hal::delay_ms(500);
        }
    }
}
