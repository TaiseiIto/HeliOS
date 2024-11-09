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

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let sub_class: u8 = register.sub_class();
        let class: u8 = register.class();
        formatter
            .debug_struct("Register")
            .field("sub_class", &sub_class)
            .field("class", &class)
            .finish()
    }
}

/// # Task Priority Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.8.3.1 Figure 11-18. Task Priority Register (TPR)
#[bitfield(u32)]
struct Register {
    #[bits(4)]
    sub_class: u8,
    #[bits(4)]
    class: u8,
    #[bits(24)]
    __: u32,
}

