//! Kernel arguments.

use {
    core::cell::OnceCell,
    crate::{
        interrupt,
        processor,
        x64,
    },
};

static mut ARGUMENT: OnceCell<Argument> = OnceCell::new();

#[derive(Clone, Debug)]
#[repr(packed)]
pub struct Argument {
    ia32_apic_base: x64::msr::ia32::ApicBase,
    message: *mut Option<processor::message::Content>,
    bsp_local_apic_id: u8,
}

impl Argument {
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

    pub fn set(self) {
        unsafe {
            ARGUMENT.set(self)
        }.unwrap()
    }

    fn message_mut(&mut self) -> &mut Option<processor::message::Content> {
        unsafe {
            &mut *self.message
        }
    }

    pub fn send_char(&mut self, character: char) {
        *self.message_mut() = Some(processor::message::Content::char(character));
        let mut ia32_apic_base: x64::msr::ia32::ApicBase = self.ia32_apic_base;
        ia32_apic_base
            .registers_mut()
            .send_interrupt(self.bsp_local_apic_id, 0x20);
    }
}

