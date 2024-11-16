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

    pub fn read() -> Self {
        x64::cmos::read_u8(Self::ADDRESS).into()
    }

    pub fn set_frequency(self, hz: usize) -> Self {
        assert!(hz.is_power_of_two());
        let log2hz: u8 = hz.ilog2() as u8;
        assert!(log2hz < 16);
        let rate: u8 = 16 - (hz.ilog2() as u8);
        assert!(rate < 16);
        self.with_rate(rate)
    }

    pub fn write(self) {
        x64::cmos::write_u8(Self::ADDRESS, self.into())
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

    pub fn correct_hour(&self, hour: u8) -> u8 {
        if self.hour_mode_24() {
            self.binarize(hour)
        } else {
            const PM: u8 = 0x80;
            let pm: bool = hour & PM != 0;
            let hour: u8 = hour & !PM;
            let hour: u8 = self.binarize(hour);
            let hour: u8 = hour % 12;
            hour + if pm {
                12
            } else {
                0
            }
        }
    }

    pub fn disable_periodic_interrupt(self) -> Self {
        self.with_pie(false)
    }

    pub fn enable_periodic_interrupt(self) -> Self {
        self.with_pie(true)
    }

    pub fn read() -> Self {
        x64::cmos::read_u8(Self::ADDRESS).into()
    }

    pub fn write(self) {
        x64::cmos::write_u8(Self::ADDRESS, self.into())
    }
}

#[bitfield(u8)]
pub struct C {
    #[bits(4)]
    __: u8,
    uf: bool,
    af: bool,
    pf: bool,
    irq: bool,
}

impl C {
    const ADDRESS: u8 = 0x0c;

    pub fn read() -> Self {
        x64::cmos::read_u8(Self::ADDRESS).into()
    }
}

#[bitfield(u8)]
pub struct D {
    #[bits(7)]
    __: u8,
    vm: bool,
}

impl D {
    const ADDRESS: u8 = 0x0d;

    pub fn read() -> Self {
        x64::cmos::read_u8(Self::ADDRESS).into()
    }
}

