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
    pub fn write(&mut self, value: u32) {
        self.register = self.register.with_end_of_interrupt(value);
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let end_of_interrupt: u32 = register.end_of_interrupt();
        formatter
            .debug_struct("Register")
            .field("end_of_interrupt", &end_of_interrupt)
            .finish()
    }
}

/// # EOI Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.8.5 Figure 11-22. EOI Register
#[bitfield(u32)]
struct Register {
    end_of_interrupt: u32,
}

