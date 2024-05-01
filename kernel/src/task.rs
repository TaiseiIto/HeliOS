use {
    alloc::vec::Vec,
    core::cell::UnsafeCell,
    crate::{
        Argument,
        x64,
    },
};

static mut CURRENT: UnsafeCell<Vec<Controller>> = UnsafeCell::new(Vec::new());

pub struct Controller {
    interrupt_disable_level: usize,
}

impl Controller {
    pub fn cli(&mut self) {
        if self.interrupt_disable_level == 0 {
            assert!(x64::Rflags::get().interrupt_is_enabled());
            x64::cli();
        }
        assert!(!x64::Rflags::get().interrupt_is_enabled());
        self.interrupt_disable_level += 1;
    }

    pub fn end_interrupt(&mut self) {
        self.interrupt_disable_level -= 1;
    }

    pub fn new() -> Self {
        let interrupt_disable_level: usize = if x64::Rflags::get().interrupt_is_enabled() {
            0
        } else {
            1
        };
        Self {
            interrupt_disable_level,
        }
    }

    pub fn start_interrupt(&mut self) {
        self.interrupt_disable_level += 1;
    }

    pub fn sti(&mut self) {
        assert!(!x64::Rflags::get().interrupt_is_enabled());
        self.interrupt_disable_level -= 1;
        if self.interrupt_disable_level == 0 {
            x64::sti();
            assert!(x64::Rflags::get().interrupt_is_enabled());
        }
    }
}

