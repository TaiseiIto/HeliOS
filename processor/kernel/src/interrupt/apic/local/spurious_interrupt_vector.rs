use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    reserved0: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let spurious_vector: u8 = register.spurious_vector();
        let apic_software_enable: bool = register.apic_software_enable();
        let focus_processor_checking_enable: bool = register.focus_processor_checking_enable();
        let eoi_broadcast_suppression: bool = register.eoi_broadcast_suppression();
        formatter
            .debug_struct("Register")
            .field("spurious_vector", &spurious_vector)
            .field("apic_software_enable", &apic_software_enable)
            .field("focus_processor_checking_enable", &focus_processor_checking_enable)
            .field("eoi_broadcast_suppression", &eoi_broadcast_suppression)
            .finish()
    }
}

/// # Spurious Interrupt Vector Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.9 Figure 11-23. Spurious Interrupt Vector Register (SVR)
#[bitfield(u32)]
struct Register {
    spurious_vector: u8,
    apic_software_enable: bool,
    focus_processor_checking_enable: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
    eoi_broadcast_suppression: bool,
    #[bits(19, access = RO)]
    reserved1: u32,
}

