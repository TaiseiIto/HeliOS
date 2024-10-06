use bitfield_struct::bitfield;

/// # Base Address Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h)
#[derive(Debug)]
pub enum Register {
    Memory(Memory),
    Io(Io),
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
    #[bits(access = RO)]
    reserved0: bool,
    #[bits(30)]
    base_address: u32,
}

