#![no_std]
#![no_main]

mod servo;


use servo::*;

use panic_halt as _;
use arduino_hal::default_serial;
use ufmt::uwriteln;

fn scale_joystick_degrees(i: u16) -> u8 {
    ((i as u32 * 100000 / 1024 * 180) / 100000) as u8
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);

    let _ = uwriteln!(&mut serial, "Initializing Arduino Mega...");
    
    let tc1 = dp.TC1;
    let mut servo = ServoPin::new(&tc1, pins.d11);
    // servo.set_mode(ServoMode::Precise);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a_x = pins.a0.into_analog_input(&mut adc);
    let a_y = pins.a1.into_analog_input(&mut adc);

    // let channels: [arduino_hal::adc::Channel; 2] = [
    //     v_x.into_channel(),
    //     v_y.into_channel(),
    // ];

    let _ = uwriteln!(&mut serial, "TCCR1A {}", tc1.tccr1a().read().bits());
    let _ = uwriteln!(&mut serial, "TCCR1B {}", tc1.tccr1b().read().bits());
    
    // Joystick min = 0
    // Joystick max = 1023
   
    loop {
        // for i in 125..625u16 {
        // for i in [0, 90, 0, 90, 0, 180, 179, 178, 160, 120] {
        //     let val = servo.set_rotation(i);
        //     // let _ = uwriteln!(&mut serial, "\n\rValue: {}", val);
        //     // let _ = uwriteln!(&mut serial, "\n\rOCR1A: {}", tc1.ocr1a().read().bits());
        //     arduino_hal::delay_ms(500);
        // }
        // for ch in channels.iter() {
        //     let adc.read_blocking(ch);
        // }
        let v_x = a_x.analog_read(&mut adc);
        let v_y = a_y.analog_read(&mut adc);
        
        let rotation = scale_joystick_degrees(v_x);
        servo.set_rotation(rotation);

        let _ = uwriteln!(&mut serial, "X: {}, Y: {}, rotation: {}", v_x, v_y, rotation);

        // arduino_hal::delay_ms(100);
    }
}
