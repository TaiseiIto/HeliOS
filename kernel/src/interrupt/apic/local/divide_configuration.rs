use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    reserved0: [u32; 3],
}

impl FatRegister {
    pub fn set_divisor(&mut self, divisor: u8) {
        self.register = self.register.set_divisor(divisor);
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let divisor: u8 = register.divisor();
        formatter
            .debug_struct("Register")
            .field("divisor", &divisor)
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
    #[bits(access = RO)]
    reserved0: bool,
    divide_value1: bool,
    #[bits(28, access = RO)]
    reserved1: u32,
}

impl Register {
    fn divide_value(&self) -> u8 {
        self.divide_value0() + if self.divide_value1() {
            0b100
        } else {
            0b000
        }
    }

    fn divisor(&self) -> u8 {
        1 << ((self.divide_value() + 1) % 0b1000)
    }

    fn set_divide_value(self, divide_value: u8) -> Self {
        assert!(divide_value < 0b1000);
        let divide_value0: u8 = divide_value & 0b011;
        let divide_value1: bool = divide_value & 0b100 != 0;
        self.with_divide_value0(divide_value0)
            .with_divide_value1(divide_value1)
    }

    fn set_divisor(self, divisor: u8) -> Self {
        assert!(divisor.is_power_of_two());
        let divide_value: u8 = (divisor.ilog2().wrapping_sub(1) & 0b111) as u8;
        self.set_divide_value(divide_value)
    }
}

