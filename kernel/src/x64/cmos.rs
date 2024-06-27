//! # CMOS
//! ## References
//! * [CMOS](https://wiki.osdev.org/CMOS)

use super::port;

const ADDRESS_PORT: u16 = 0x0070;
const DATA_PORT: u16 = 0x0071;

pub fn read(address: u8) -> u8 {
    assert!(address < 0x80);
    port::outb(ADDRESS_PORT, address);
    port::inb(DATA_PORT)
}

pub fn write(address: u8, value: u8) {
    assert!(address < 0x80);
    port::outb(ADDRESS_PORT, address);
    port::outb(DATA_PORT, value);
}

