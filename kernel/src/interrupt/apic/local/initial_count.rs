use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    __: [u32; 3],
}

impl FatRegister {
    pub fn set(&mut self, initial_count: u32) {
        let register: Register = self.register;
        let register: Register = register.with_initial_count(initial_count);
        *self.register_mut() = register.into();
    }

    fn register_mut(&mut self) -> &mut u32 {
        let address: *mut Self = self as *mut Self;
        let address: *mut u32 = address as *mut u32;
        unsafe {
            &mut *address
        }
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let initial_count: u32 = register.initial_count();
        formatter
            .debug_struct("Register")
            .field("initial_count", &initial_count)
            .finish()
    }
}

/// # Initial Count Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.4 Figure 11-11. Initial Count and Current Count Registers
#[bitfield(u32)]
struct Register {
    initial_count: u32,
}

