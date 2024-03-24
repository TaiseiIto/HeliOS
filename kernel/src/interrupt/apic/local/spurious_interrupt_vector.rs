use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
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

