use bitfield_struct::bitfield;

/// # IRR, ISR and TMR Registers
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.8.4 Figure 11-20. IRR, ISR, and TMR Register
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegisters {
    registers: [FatRegister; 8],
}

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

#[bitfield(u32)]
struct Register {
    register: u32,
}

