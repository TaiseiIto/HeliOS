//! # CMOS
//! ## References
//! * [CMOS](https://wiki.osdev.org/CMOS)

use {
    crate::interrupt,
    super::port,
};

pub const ADDRESS_PORT: u16 = 0x0070;
pub const DATA_PORT: u16 = 0x0071;

pub fn read_u8(address: u8) -> u8 {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    let address = address | if interrupt::non_maskable::is_enabled() {
        0
    } else {
        interrupt::non_maskable::DISABLE
    };
    port::outb(ADDRESS_PORT, address);
    port::inb(DATA_PORT)
}

pub fn read_u16(address: u8) -> u16 {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    let address = address | if interrupt::non_maskable::is_enabled() {
        0
    } else {
        interrupt::non_maskable::DISABLE
    };
    port::outb(ADDRESS_PORT, address);
    port::inw(DATA_PORT)
}

pub fn read_u32(address: u8) -> u32 {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    let address = address | if interrupt::non_maskable::is_enabled() {
        0
    } else {
        interrupt::non_maskable::DISABLE
    };
    port::outb(ADDRESS_PORT, address);
    port::inl(DATA_PORT)
}

pub fn write_u8(address: u8, value: u8) {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    let address = address | if interrupt::non_maskable::is_enabled() {
        0
    } else {
        interrupt::non_maskable::DISABLE
    };
    port::outb(ADDRESS_PORT, address);
    port::outb(DATA_PORT, value);
}

pub fn write_u16(address: u8, value: u16) {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    let address = address | if interrupt::non_maskable::is_enabled() {
        0
    } else {
        interrupt::non_maskable::DISABLE
    };
    port::outb(ADDRESS_PORT, address);
    port::outw(DATA_PORT, value);
}

pub fn write_u32(address: u8, value: u32) {
    assert_eq!(address & interrupt::non_maskable::DISABLE, 0);
    let address = address | if interrupt::non_maskable::is_enabled() {
        0
    } else {
        interrupt::non_maskable::DISABLE
    };
    port::outb(ADDRESS_PORT, address);
    port::outl(DATA_PORT, value);
}

