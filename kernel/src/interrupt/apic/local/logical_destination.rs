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
        let logical_apic_id: u8 = register.logical_apic_id();
        formatter
            .debug_struct("Register")
            .field("logical_apic_id", &logical_apic_id)
            .finish()
    }
}

/// # Logical Destination Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.2.2 Figure 11-13. Logical Destination Register (LDR)
#[bitfield(u32)]
struct Register {
    #[bits(24)]
    __: u32,
    logical_apic_id: u8,
}

