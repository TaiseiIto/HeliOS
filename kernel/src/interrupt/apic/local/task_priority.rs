use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
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
    #[bits(24, access = RO)]
    reserved0: u32,
}

