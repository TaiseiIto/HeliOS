use bitfield_struct::bitfield;

/// # Base Address Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h)
#[derive(Debug)]
pub enum Register {
    Memory(Memory),
    Io(Io),
}

impl Register {
    pub fn read_memory<T>(&self) -> Option<&T> {
        match self {
            Self::Memory(memory) => {
                let lower_address: u32 = memory
                    .clone()
                    .with_memory_space_indicator(false)
                    .with_memory_type(0)
                    .with_prefetchable(false)
                    .into();
                let lower_register: *const Memory = memory as *const Memory;
                let higher_register: *const Memory = unsafe {
                    lower_register.add(1)
                };
                let higher_address: Memory = unsafe {
                    *higher_register
                };
                let higher_address: u32 = higher_address.into();
                let address: usize = ((higher_address as usize) << u32::BITS) | (lower_address as usize);
                let address: *const T = address as *const T;
                let address: &T = unsafe {
                    &*address
                };
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

/// # Base Address Register for Memory
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h) Figure 7-11 Base Address Register for Memory
#[bitfield(u32)]
pub struct Memory {
    memory_space_indicator: bool,
    #[bits(2)]
    memory_type: u8,
    prefetchable: bool,
    #[bits(28)]
    base_address: u32,
}

/// # Base Address Register for I/O
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h) Figure 7-12 Base Address Register for I/O
#[bitfield(u32)]
pub struct Io {
    io_space_indicator: bool,
    __: bool,
    #[bits(30)]
    base_address: u32,
}

