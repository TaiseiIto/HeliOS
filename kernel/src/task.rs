use {
    alloc::vec::Vec,
    core::cell::UnsafeCell,
    crate::x64,
};

static mut ALL: UnsafeCell<Vec<Controller>> = UnsafeCell::new(Vec::new());

pub struct Controller {
    current: bool,
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
    
    pub fn get_current_mut() -> Option<&'static mut Self> {
        unsafe {
            ALL
                .get_mut()
                .iter_mut()
                .find(|controller| controller.current)
        }
    }

    pub fn set_current() {
        let current: bool = true;
        let interrupt_disable_level: usize = if x64::Rflags::get().interrupt_is_enabled() {
            0
        } else {
            1
        };
        let current = Self {
            current,
            interrupt_disable_level,
        };
        unsafe {
            ALL.get_mut().push(current);
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

