use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

/// # Destinatio Format Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.2.2 Figure 11-14. Destination Format Register (DFR)
#[bitfield(u32)]
struct Register {
    #[bits(28, access = RO)]
    reserved0: u32,
    #[bits(4)]
    model: u8,
}

