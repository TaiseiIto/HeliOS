//! # RTC Status Registers
//! ## References
//! * [CMOS Registers](http://www.walshcomptech.com/ohlandl/config/cmos_registers.html)

use {
    bitfield_struct::bitfield,
    crate::x64,
};

#[bitfield(u8)]
pub struct A {
    #[bits(4)]
    rate: u8,
    bc: bool,
    #[bits(2)]
    sdiv: u8,
    uip: bool,
}

impl A {
    const ADDRESS: u8 = 0x0a;

    pub fn get() -> Self {
        x64::cmos::read(Self::ADDRESS).into()
    }
}

#[bitfield(u8)]
pub struct B {
    dse: bool,
    hour_mode_24: bool,
    dm: bool,
    sqw: bool,
    uie: bool,
    aie: bool,
    pie: bool,
    set: bool,
}

impl B {
    const ADDRESS: u8 = 0x0b;

    pub fn binarize(&self, value: u8) -> u8 {
        if self.dm() {
            value
        } else {
            let ones_place: u8 = value % 0x10;
            assert!(ones_place < 10);
            let tens_place: u8 = value / 0x10;
            assert!(tens_place < 10);
            10 * tens_place + ones_place
        }
    }

    pub fn get() -> Self {
        x64::cmos::read(Self::ADDRESS).into()
    }
}

#[bitfield(u8)]
pub struct C {
    #[bits(4, access = RO)]
    reserved0: u8,
    uf: bool,
    af: bool,
    pf: bool,
    irq: bool,
}

impl C {
    const ADDRESS: u8 = 0x0c;

    pub fn get() -> Self {
        x64::cmos::read(Self::ADDRESS).into()
    }
}

#[bitfield(u8)]
pub struct D {
    #[bits(7, access = RO)]
    reserved0: u8,
    vm: bool,
}

impl D {
    const ADDRESS: u8 = 0x0d;

    pub fn get() -> Self {
        x64::cmos::read(Self::ADDRESS).into()
    }
}

