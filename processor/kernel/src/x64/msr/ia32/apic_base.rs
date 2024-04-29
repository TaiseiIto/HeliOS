use bitfield_struct::bitfield;

/// # IA32_APIC_BASE
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-4
#[bitfield(u64)]
pub struct ApicBase {
    #[bits(access = RO)]
    reserved0: u8,
    bsp: bool,
    #[bits(access = RO)]
    reserved1: bool,
    enable_x2apic_mode: bool,
    apic_global_enable: bool,
    #[bits(52)]
    apic_base: u64,
}

