use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

/// # Arbitration Priority Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.2.4 Figure 11-15. Arbitration Priority Register (APR)
#[bitfield(u32)]
struct Register {
    #[bits(4)]
    sub_class: u8,
    #[bits(4)]
    class: u8,
    #[bits(24, access = RO)]
    reserved0: u32,
}

