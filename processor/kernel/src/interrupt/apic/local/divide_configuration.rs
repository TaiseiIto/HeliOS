use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    __: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let divide_value: u8 = register.divide_value();
        formatter
            .debug_struct("Register")
            .field("divide_value", &divide_value)
            .finish()
    }
}

/// # Divide Configuration Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.4 Figure 11-10. Divide Configuration Register
#[bitfield(u32)]
struct Register {
    #[bits(2)]
    divide_value0: u8,
    __: bool,
    divide_value1: bool,
    #[bits(28)]
    __: u32,
}

impl Register {
    fn divide_value(&self) -> u8 {
        self.divide_value0() + if self.divide_value1() {
            4
        } else {
            0
        }
    }
}

