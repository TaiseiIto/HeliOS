//! Kernel arguments.

use {
    core::{
        cell::OnceCell,
        fmt::{
            self,
            Write,
        },
    },
    crate::{
        interrupt,
        processor,
        x64,
    },
};

static mut ARGUMENT: OnceCell<Argument> = OnceCell::new();

#[macro_export]
macro_rules! bsp_println {
    ($fmt:expr) => (bsp_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (bsp_print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! bsp_print {
    ($($arg:tt)*) => ($crate::argument::bsp_print(format_args!($($arg)*)));
}

pub fn bsp_print(args: fmt::Arguments) {
    Argument::get_mut()
        .write_fmt(args)
        .unwrap()
}

#[derive(Clone, Debug)]
#[repr(packed)]
pub struct Argument {
    ia32_apic_base: x64::msr::ia32::ApicBase,
    message: *mut Option<processor::message::Content>,
    bsp_local_apic_id: u8,
}

impl Argument {
    pub fn boot_completed(&mut self) {
        while self.message().is_some() {
            x64::pause();
        }
        *self.message_mut() = Some(processor::message::Content::boot_completed());
        let mut ia32_apic_base: x64::msr::ia32::ApicBase = self.ia32_apic_base;
        ia32_apic_base
            .registers_mut()
            .send_interrupt(self.bsp_local_apic_id, interrupt::INTERPROCESSOR_INTERRUPT);
    }

    pub fn get() -> &'static Self {
        unsafe {
            ARGUMENT
                .get()
                .unwrap()
        }
    }

    pub fn get_mut() -> &'static mut Self {
        unsafe {
            ARGUMENT
                .get_mut()
                .unwrap()
        }
    }

    pub fn send_char(&mut self, character: char) {
        while self.message().is_some() {
            x64::pause();
        }
        *self.message_mut() = Some(processor::message::Content::char(character));
        let mut ia32_apic_base: x64::msr::ia32::ApicBase = self.ia32_apic_base;
        ia32_apic_base
            .registers_mut()
            .send_interrupt(self.bsp_local_apic_id, interrupt::INTERPROCESSOR_INTERRUPT);
    }

    pub fn set(self) {
        unsafe {
            ARGUMENT.set(self)
        }.unwrap()
    }

    fn message(&self) -> &Option<processor::message::Content> {
        unsafe {
            &*self.message
        }
    }

    fn message_mut(&mut self) -> &mut Option<processor::message::Content> {
        unsafe {
            &mut *self.message
        }
    }

}

impl fmt::Write for Argument {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        string
            .chars()
            .for_each(|character| self.send_char(character));
        Ok(())
    }
}

