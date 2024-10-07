use {
    bitfield_struct::bitfield,
    super::Function,
};

/// # Header Type Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.9 Command Register (Offset 0Eh)
#[bitfield(u8)]
pub struct Register {
    #[bits(7)]
    header_layout: u8,
    multi_function_device: bool,
}

impl Register {
    pub fn is_multi_function_device(&self) -> bool {
        self.multi_function_device()
    }
}

pub enum Type {
    Zero,
    One,
}

impl From<&Register> for Type {
    fn from(register: &Register) -> Self {
        match register.header_layout() {
            0x00 => Self::Zero,
            0x01 => Self::One,
            _ => unreachable!(),
        }
    }
}

impl From<&Function> for Type {
    fn from(function: &Function) -> Self {
        let register: Register = function.header_type();
        (&register).into()
    }
}

