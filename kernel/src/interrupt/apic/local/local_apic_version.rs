use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

/// # Local APIC Version Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.4.8 Local APIC Version Register
#[bitfield(u32)]
struct Register {
    version: u8,
    #[bits(access = RO)]
    reserved0: u8,
    max_lvt_entry: u8,
    support_for_eoi_broadcast_suppression: bool,
    #[bits(7, access = RO)]
    reserved1: u8,
}

