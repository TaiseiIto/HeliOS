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

impl FatRegister {
    pub fn apic_id(&self) -> u8 {
        let register: Register = self.register;
        register.apic_id()
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let apic_id: u8 = register.apic_id();
        formatter
            .debug_struct("Register")
            .field("apic_id", &apic_id)
            .finish()
    }
}

/// # Local APIC ID Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.4.6 Local APIC ID Register
#[bitfield(u32)]
struct Register {
    #[bits(24, access = RO)]
    reserved0: u32,
    apic_id: u8,
}

