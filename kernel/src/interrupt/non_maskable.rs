//! # Non Maskable Interrupt
//! ## References
//! * [Non Maskable Interrupt](https://wiki.osdev.org/Non_Maskable_Interrupt)

use crate::x64;

pub const DISABLE: u8 = 0x80;

pub fn disable() {
    let address: u8 = x64::port::inb(x64::cmos::ADDRESS_PORT);
    let address: u8 = address | DISABLE;
    x64::port::outb(x64::cmos::ADDRESS_PORT, address);
    x64::port::inb(x64::cmos::DATA_PORT);
}

pub fn enable() {
    let address: u8 = x64::port::inb(x64::cmos::ADDRESS_PORT);
    let address: u8 = address & !DISABLE;
    x64::port::outb(x64::cmos::ADDRESS_PORT, address);
    x64::port::inb(x64::cmos::DATA_PORT);
}

