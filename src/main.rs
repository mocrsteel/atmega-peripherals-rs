#![no_std]
#![no_main]

mod servo;


use servo::*;

use panic_halt as _;
use arduino_hal::default_serial;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);

    let _ = uwriteln!(&mut serial, "Initializing Arduino Mega...");
    // // pin 11 PWM setup TC1 channel A
    // let tc1 = dp.TC1;
    // tc1.tccr1a().reset();
    // tc1.tccr1b().reset();
    // tc1.tccr1c().reset();
    // // Compare mode set-up in register TCCR1A
    // // - bits 7:6 - COM1A1:0 = Output compare mode for OC1A (for channel A)
    // // - bits 5:4 - COM1B1:0 = Output compare mode for OC1B (for channel B)
    // // - bits 3:2 - COM1C1:0 = Output compare mode for OC1C (for channel C)
    // // WGM 010 => FastPWM with ICRn to define TOP and OCRNX to define compare output.
    // tc1.tccr1a().write(|w| unsafe {w.com1a().bits(0b10).wgm1().bits(0b10)}); // non-inverting mode Compare output mode for fast-pwm
    // tc1.tccr1b().write(|w| unsafe {w.wgm1().bits(0b11).cs1().prescale_8()});
    
    // // Setting TOP to IRC1 to achieve 50 Hz cycle.
    // tc1.icr1().write(|w| w.set(39999u16));
    
    // // Setting output compare on channel C to define the PWM duty cycle.
    // tc1.ocr1a().write(|w| w.set(0u16));
    
    // // Toggle pin 11 to output the OC3A output.
    // pins.d11.into_output();
    
    // // pin 3 PWM setup with TC3 channel B
    let tc3 = dp.TC3;
    tc3.tccr3a().reset();
    tc3.tccr3b().reset();
    tc3.tccr3c().reset();
    tc3.tccr3a().write(|w| unsafe{ w.com3a().bits(0b10).wgm3().bits(0b10)});
    tc3.tccr3b().write(|w| unsafe{ w.wgm3().bits(0b11).cs3().prescale_64()});
    
    // Setting TOP to define PWM cycle.
    tc3.icr3().write(|w| w.set(4999u16));
    
    // Setting duty cycle on channel C.
    tc3.ocr3a().write(|w| w.set(0u16));
    
    pins.d3.into_output();
    
    let ocr3c_val = tc3.ocr3c().read().bits();
    
    // let _ = ufmt::uwrite!(&mut serial, "\n\rOCR1A = {}, READ = {}", i, ocr3c_val);
    // let servo = Servo::into_servo();
    // let _ = debug_dump(&mut serial, &tc1);
    // SG90 0degrees = 86
    // SG90 180 degrees = 660
    let tc1 = dp.TC1;
    let servo = ServoPin::new(&tc1, pins.d11);


    let _ = uwriteln!(&mut serial, "TCCR1A {}", tc1.tccr1a().read().bits());
    let _ = uwriteln!(&mut serial, "TCCR3A {}", tc3.tccr3a().read().bits());
    let _ = uwriteln!(&mut serial, "TCCR1B {}", tc1.tccr1b().read().bits());
    let _ = uwriteln!(&mut serial, "TCCR3B {}", tc3.tccr3b().read().bits());

    
    loop {
        // for i in 125..625u16 {
        let val = servo.set_rotation(0);
        let _ = uwriteln!(&mut serial, "\n\rValue: {}", val);
        let _ = uwriteln!(&mut serial, "\n\rOCR1A: {}", tc1.ocr1a().read().bits());
        arduino_hal::delay_ms(1000);

        let val = servo.set_rotation(90);
        let _ = uwriteln!(&mut serial, "\n\rValue: {}", val);
        let _ = uwriteln!(&mut serial, "\n\rOCR1A: {}", tc1.ocr1a().read().bits());
        arduino_hal::delay_ms(1000);
        
        // }
        // for i in (125..625u16).rev() {
            let val = servo.set_rotation(180);
            let _ = uwriteln!(&mut serial, "\n\rValue: {}", val);
            let _ = uwriteln!(&mut serial, "\n\rOCR1A: {}", tc1.ocr1a().read().bits());
        // let _ = uwriteln!(&mut serial, "\n\rOCR1A: {}", tc1.ocr1a().read().bits());
        arduino_hal::delay_ms(1000);
        // }
        // // arduino_hal::delay_ms(2000);
        // for i in 0..499u16.reverse_bits() {
        //     tc1.ocr1a().write(|w| w.set(i));
        //     arduino_hal::delay_ms(25);
        //     let _ = ufmt::uwrite!(&mut serial, "\n\rOCR1A = {}", i);
        // }
    }
}
