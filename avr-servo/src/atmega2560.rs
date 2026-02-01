use arduino_hal::hal::port::{PB7, PB6, PB5, PE5, PE4, PE3, PH5, PH4, PH3, PL5, PL4, PL3};
use arduino_hal::pac::{TC1, TC3, TC4, TC5};
use crate::avr_servo::*;
use crate::impl_servo;

impl_servo! {
        /// * Port: PB7
        /// * Timer/Counter1 (16-bit)
        /// * Output compare channel C
        /// * Pin D13
        pin: PB7,
        tc: 1,
        channel: c,
}
impl_servo! {
        /// * Port: PB6
        /// * Timer/Counter1 (16-bit)
        /// * Output compare channel B
        /// * Pin D12
        pin: PB6,
        tc: 1,
        channel: b,
}
impl_servo! {
        /// * Port: PB5
        /// * Timer/Counter1 (16-bit)
        /// * Output compare channel A
        /// * Pin D11
        pin: PB5,
        tc: 1,
        channel: a,
}

impl_servo! {
        /// * Port: PE5
        /// * Timer/Counter3 (16-bit)
        /// * Output compare channel C
        /// * Pin D3
        pin: PE5,
        tc: 3,
        channel: c,
}

impl_servo! {
        /// * Port: PE4
        /// * Timer/Counter3 (16-bit)
        /// * Output compare channel B
        /// * Pin D2
        pin: PE4,
        tc: 3,
        channel: b,
}

impl_servo! {
        /// * Port: PE3
        /// * Timer/Counter3 (16-bit)
        /// * Output compare channel A
        /// * Pin D5
        pin: PE3,
        tc: 3,
        channel: a,
}

impl_servo! {
        /// * Port: PH5
        /// * Timer/Counter4 (16-bit)
        /// * Output compare channel C
        /// * Pin D8
        pin: PH5,
        tc: 4,
        channel: c,
}

impl_servo! {
        /// * Port: PH4
        /// * Timer/Counter4 (16-bit)
        /// * Output compare channel B
        /// * Pin D7
        pin: PH4,
        tc: 4,
        channel: b,
}

impl_servo! {
        /// * Port: PH3
        /// * Timer/Counter4 (16-bit)
        /// * Output compare channel A
        /// * Pin D6
        pin: PH3,
        tc: 4,
        channel: a,
}

impl_servo! {
        /// * Port: PL5
        /// * Timer/Counter5 (16-bit)
        /// * Output compare channel C
        /// * Pin D44
        pin: PL5,
        tc: 5,
        channel: c,
}

impl_servo! {
        /// * Port: PL4
        /// * Timer/Counter5 (16-bit)
        /// * Output compare channel B
        /// * Pin D45
        pin: PL4,
        tc: 5,
        channel: b,
}

impl_servo! {
        /// * Port: PL3
        /// * Timer/Counter5 (16-bit)
        /// * Output compare channel A
        /// * Pin D46
        pin: PL3,
        tc: 5,
        channel: a,
}

