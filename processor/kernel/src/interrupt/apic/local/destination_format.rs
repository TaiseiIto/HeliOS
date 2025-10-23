use {bitfield_struct::bitfield, core::fmt};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    __: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let model: u8 = register.model();
        formatter
            .debug_struct("Register")
            .field("model", &model)
            .finish()
    }
}

/// # Destinatio Format Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.2.2 Figure 11-14. Destination Format Register (DFR)
#[bitfield(u32)]
struct Register {
    #[bits(28)]
    __: u32,
    #[bits(4)]
    model: u8,
}
