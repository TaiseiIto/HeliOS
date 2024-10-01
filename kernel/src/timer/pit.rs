//! # Programmable Interval Timer
//! ## References
//! * [Programmable Interval Timer](https://wiki.osdev.org/Programmable_Interval_Timer)
//! * [8254 PROGRAMMABLE INTERVAL TIMER](https://www.scs.stanford.edu/10wi-cs140/pintos/specs/8254.pdf)

use crate::x64;

pub mod control;

const BASE_OSCILATOR_FREQUENCY: usize = 14318180;

pub fn enable_periodic_interrupt(hz: usize) -> u8 {
    let counter: u8 = 0;
    let irq: u8 = 0;
    let divisor: usize = BASE_OSCILATOR_FREQUENCY / (12 * hz);
    assert!(divisor < 0x10000);
    let divisor: u16 = divisor as u16;
    let low: u8 = divisor as u8;
    let high: u8 = (divisor >> u8::BITS) as u8;
    let bcd: bool = false;
    control::Register::create(bcd, control::Mode::RateGenerator, control::Access::LowAndHigh, control::Selector::Counter(counter)).set();
    x64::port::outb(counter_port(counter), low);
    x64::port::outb(counter_port(counter), high);
    irq
}

fn counter_port(index: u8) -> u16 {
    (index as u16) + 0x40
}

