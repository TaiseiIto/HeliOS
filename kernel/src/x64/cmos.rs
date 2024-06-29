//! # CMOS
//! ## References
//! * [CMOS](https://wiki.osdev.org/CMOS)

use {
    crate::interrupt,
    super::port,
};

pub const ADDRESS_PORT: u16 = 0x0070;
pub const DATA_PORT: u16 = 0x0071;

pub fn read(address: u8) -> u8 {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    port::outb(ADDRESS_PORT, address);
    port::inb(DATA_PORT)
}

pub fn write(address: u8, value: u8) {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    port::outb(ADDRESS_PORT, address);
    port::outb(DATA_PORT, value);
}

