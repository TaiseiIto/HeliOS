use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::fmt,
};

/// # IRR, ISR and TMR Registers
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.8.4 Figure 11-20. IRR, ISR, and TMR Register
#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegisters {
    registers: [FatRegister; 8],
}

impl FatRegisters {
    fn registers(&self) -> [u32; 8] {
        let registers: [FatRegister; 8] = self.registers;
        let registers: Vec<u32> = registers
            .into_iter()
            .map(|register| register.register())
            .collect();
        registers
            .try_into()
            .unwrap()
    }
}

impl fmt::Debug for FatRegisters {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.registers())
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

impl FatRegister {
    fn register(&self) -> u32 {
        let register: Register = self.register;
        register.register()
    }
}

#[bitfield(u32)]
struct Register {
    register: u32,
}

