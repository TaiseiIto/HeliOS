use {bitfield_struct::bitfield, core::fmt};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    __: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let version: u8 = register.version();
        let max_lvt_entry: u8 = register.max_lvt_entry();
        let support_for_eoi_broadcast_suppression: bool =
            register.support_for_eoi_broadcast_suppression();
        formatter
            .debug_struct("Register")
            .field("version", &version)
            .field("max_lvt_entry", &max_lvt_entry)
            .field(
                "support_for_eoi_broatcast_suppression",
                &support_for_eoi_broadcast_suppression,
            )
            .finish()
    }
}

/// # Local APIC Version Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.4.8 Local APIC Version Register
#[bitfield(u32)]
struct Register {
    version: u8,
    __: u8,
    max_lvt_entry: u8,
    support_for_eoi_broadcast_suppression: bool,
    #[bits(7)]
    __: u8,
}
