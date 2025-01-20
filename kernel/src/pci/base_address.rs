use bitfield_struct::bitfield;

/// # Base Address Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h)
#[derive(Debug)]
pub enum Register {
    Io(Io),
    Memory(Memory),
}

impl Register {
    pub fn memory_address(&self, next: Option<&Self>) -> Option<usize> {
        match self {
            Self::Memory(memory) => {
                let low_address: u32 = memory.base_address() << Memory::BASE_ADDRESS_OFFSET;
                let low_address: usize = low_address as usize;
                let high_address: u32 = match memory.size() {
                    Size::Bits32 => 0,
                    Size::Bits64 => next
                        .unwrap()
                        .into(),
                };
                let high_address: usize = high_address as usize;
                let address: usize = low_address + (high_address << u32::BITS);
                Some(address)
            },
            Self::Io(_) => None,
        }
    }
}

impl From<u32> for Register {
    fn from(register: u32) -> Self {
        match register & 0x00000001 {
            0x00000000 => Self::Memory(register.into()),
            0x00000001 => Self::Io(register.into()),
            _ => unreachable!(),
        }
    }
}

impl From<&Register> for u32 {
    fn from(register: &Register) -> Self {
        match register {
            Register::Io(io) => io
                .clone()
                .into(),
            Register::Memory(memory) => memory
                .clone()
                .into(),
        }
    }
}

/// # Base Address Register for I/O
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h) Figure 7-12 Base Address Register for I/O
#[bitfield(u32)]
struct Io {
    io_space_indicator: bool,
    __: bool,
    #[bits(30)]
    base_address: u32,
}

/// # Base Address Register for Memory
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h) Figure 7-11 Base Address Register for Memory
#[bitfield(u32)]
struct Memory {
    memory_space_indicator: bool,
    #[bits(2)]
    memory_type: u8,
    prefetchable: bool,
    #[bits(28)]
    base_address: u32,
}

impl Memory {
    fn size(&self) -> Size {
        self.memory_type()
            .into()
    }
}

enum Size {
    Bits32,
    Bits64,
}

impl From<u8> for Size {
    fn from(size: u8) -> Self {
        match size {
            0b00 => Self::Bits32,
            0b10 => Self::Bits64,
            _ => unimplemented!(),
        }
    }
}

impl From<Size> for u8 {
    fn from(size: Size) -> Self {
        match size {
            Size::Bits32 => 0b00,
            Size::Bits64 => 0x10,
        }
    }
}

