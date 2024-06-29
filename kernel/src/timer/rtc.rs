//! # RTC
//! ## References
//! * [RTC](https://wiki.osdev.org/RTC)
//! * [CMOS](https://wiki.osdev.org/CMOS)
//! * [CMOS Registers](http://www.walshcomptech.com/ohlandl/config/cmos_registers.html)

pub mod status_register;

use {
    core::fmt,
    crate::{
        Argument,
        interrupt,
        task,
        x64,
    },
};

pub fn end_interruption() {
    status_register::C::read();
}

pub fn set_periodic_interrupt(hz: usize) -> u8 {
    let irq: u8 = 8;
    task::Controller::get_current_mut()
        .unwrap()
        .cli();
    interrupt::non_maskable::disable();
    status_register::A::read()
        .set_frequency(hz)
        .write();
    status_register::B::read()
        .enable_periodic_interrupt()
        .write();
    end_interruption();
    interrupt::non_maskable::enable();
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
    irq
}

pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
    day_of_week: DayOfWeak,
    day_of_month: u8,
    month: u8,
    year: usize,
}

impl Time {
    const SECOND_ADDRESS: u8 = 0x00;
    const MINUTE_ADDRESS: u8 = 0x02;
    const HOUR_ADDRESS: u8 = 0x04;
    const DAY_OF_WEEK_ADDRESS: u8 = 0x06;
    const DAY_OF_MONTH_ADDRESS: u8 = 0x07;
    const MONTH_ADDRESS: u8 = 0x08;
    const YEAR_ADDRESS: u8 = 0x09;

    pub fn get() -> Self {
        let status_register_b = status_register::B::read();
        let second: u8 = status_register_b.binarize(x64::cmos::read(Self::SECOND_ADDRESS));
        let minute: u8 = status_register_b.binarize(x64::cmos::read(Self::MINUTE_ADDRESS));
        let hour: u8 = status_register_b.correct_hour(x64::cmos::read(Self::HOUR_ADDRESS));
        let day_of_week: DayOfWeak = status_register_b.binarize(x64::cmos::read(Self::DAY_OF_WEEK_ADDRESS)).into();
        let day_of_month: u8 = status_register_b.binarize(x64::cmos::read(Self::DAY_OF_MONTH_ADDRESS));
        let month: u8 = status_register_b.binarize(x64::cmos::read(Self::MONTH_ADDRESS));
        let year: u8 = status_register_b.binarize(x64::cmos::read(Self::YEAR_ADDRESS));
        let century: u8 = Argument::get()
            .efi_system_table()
            .rsdp()
            .xsdt()
            .fadt()
            .century();
        let century: Option<u8> = (century != 0).then(|| status_register_b.binarize(x64::cmos::read(century)));
        let year: usize = (year as usize) + 100 * (century.unwrap_or(0) as usize);
        Self {
            second,
            minute,
            hour,
            day_of_week,
            day_of_month,
            month,
            year,
        }
    }

}

impl fmt::Debug for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#?}/{:#?}/{:#?} {:#?} {:#?}:{:#?}:{:#?}", self.year, self.month, self.day_of_month, self.day_of_week, self.hour, self.minute, self.second)
    }
}

#[derive(Debug)]
pub enum DayOfWeak {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl From<u8> for DayOfWeak {
    fn from(day_of_week: u8) -> Self {
        match day_of_week {
            1 => Self::Sunday,
            2 => Self::Monday,
            3 => Self::Tuesday,
            4 => Self::Wednesday,
            5 => Self::Thursday,
            6 => Self::Friday,
            7 => Self::Saturday,
            _ => unreachable!(),
        }
    }
}

