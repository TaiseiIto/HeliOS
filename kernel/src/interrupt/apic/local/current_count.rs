use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let current_count: u32 = register.current_count();
        formatter
            .debug_struct("Register")
            .field("current_count", &current_count)
            .finish()
    }
}

/// # Current Count Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.4 Figure 11-11. Initial Count and Current Count Registers
#[bitfield(u32)]
struct Register {
    current_count: u32,
}

