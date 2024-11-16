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

pub fn disable_periodic_interrupt() {
    task::Controller::get_current_mut()
        .unwrap()
        .cli();
    interrupt::non_maskable::disable();
    status_register::B::read()
        .disable_periodic_interrupt()
        .write();
    interrupt::non_maskable::enable();
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
}

pub fn enable_periodic_interrupt(hz: usize) -> u8 {
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

pub fn end_interruption() {
    status_register::C::read();
}

pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: usize,
}

impl Time {
    const SECOND_ADDRESS: u8 = 0x00;
    const MINUTE_ADDRESS: u8 = 0x02;
    const HOUR_ADDRESS: u8 = 0x04;
    const DAY_ADDRESS: u8 = 0x07;
    const MONTH_ADDRESS: u8 = 0x08;
    const YEAR_ADDRESS: u8 = 0x09;

    pub fn get() -> Self {
        let status_register_b = status_register::B::read();
        let second: u8 = status_register_b.binarize(x64::cmos::read_u8(Self::SECOND_ADDRESS));
        let minute: u8 = status_register_b.binarize(x64::cmos::read_u8(Self::MINUTE_ADDRESS));
        let hour: u8 = status_register_b.correct_hour(x64::cmos::read_u8(Self::HOUR_ADDRESS));
        let day: u8 = status_register_b.binarize(x64::cmos::read_u8(Self::DAY_ADDRESS));
        let month: u8 = status_register_b.binarize(x64::cmos::read_u8(Self::MONTH_ADDRESS));
        let year: u8 = status_register_b.binarize(x64::cmos::read_u8(Self::YEAR_ADDRESS));
        let century: u8 = Argument::get()
            .efi_system_table()
            .rsdp()
            .xsdt()
            .fadt()
            .century();
        let century: Option<u8> = (century != 0).then(|| status_register_b.binarize(x64::cmos::read_u8(century)));
        let year: usize = (year as usize) + 100 * (century.unwrap_or(0) as usize);
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

impl fmt::Debug for Time {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let day_of_week: DayOfWeek = self.into();
        write!(formatter, "{:#?}/{:#?}/{:#?} {:#?} {:#?}:{:#?}:{:#?}", self.year, self.month, self.day, day_of_week, self.hour, self.minute, self.second)
    }
}

#[derive(Debug)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// # References
/// * [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
impl From<&Time> for DayOfWeek {
    fn from(time: &Time) -> Self {
        let Time {
            second: _,
            minute: _,
            hour: _,
            day,
            month,
            year,
        } = time;
        let q: usize = *day as usize;
        let (m, year): (usize, usize) = match month {
            1 | 2 => ((*month as usize) + 12, *year - 1),
            _ => (*month as usize, *year),
        };
        let (k, j): (usize, usize) = (year % 100, year / 100);
        let h: usize = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        match h {
            0 => Self::Saturday,
            1 => Self::Sunday,
            2 => Self::Monday,
            3 => Self::Tuesday,
            4 => Self::Wednesday,
            5 => Self::Thursday,
            6 => Self::Friday,
            _ => unreachable!(),
        }
    }
}

