use bitfield_struct::bitfield;

/// # IRR, ISR and TMR Registers
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.8.4 Figure 11-20. IRR, ISR, and TMR Register
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegisters {
    #[allow(dead_code)]
    registers: [FatRegister; 8],
}

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
struct FatRegister {
    #[allow(dead_code)]
    register: Register,
    #[allow(dead_code)]
    __: [u32; 3],
}

impl FatRegister {
    #[allow(dead_code)]
    fn register(&self) -> u32 {
        let register: Register = self.register;
        register.register()
    }
}

#[bitfield(u32)]
struct Register {
    register: u32,
}

