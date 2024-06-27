//! # RTC
//! ## References
//! * [CMOS](https://wiki.osdev.org/CMOS)
//! * [RTC](https://wiki.osdev.org/RTC)

use crate::x64;

#[derive(Debug)]
pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u8,
}

impl Time {
    const SECOND_ADDRESS: u8 = 0x00;
    const MINUTE_ADDRESS: u8 = 0x02;
    const HOUR_ADDRESS: u8 = 0x04;
    const DAY_ADDRESS: u8 = 0x07;
    const MONTH_ADDRESS: u8 = 0x08;
    const YEAR_ADDRESS: u8 = 0x09;

    pub fn get() -> Self {
        let second: u8 = x64::cmos::read(Self::SECOND_ADDRESS);
        let minute: u8 = x64::cmos::read(Self::MINUTE_ADDRESS);
        let hour: u8 = x64::cmos::read(Self::HOUR_ADDRESS);
        let day: u8 = x64::cmos::read(Self::DAY_ADDRESS);
        let month: u8 = x64::cmos::read(Self::MONTH_ADDRESS);
        let year: u8 = x64::cmos::read(Self::YEAR_ADDRESS);
        Self {
            second,
            minute,
            hour,
            day,
            month,
            year,
        }
    }
}

