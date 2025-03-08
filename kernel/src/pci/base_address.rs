use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::fmt,
};

/// # Base Addresses
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h)
pub struct Addresses(Vec<Address>);

impl Addresses {
    pub fn iter(&self) -> impl Iterator<Item = &Address> {
        let Self(addresses) = self;
        addresses.iter()
    }
}

impl From<&[u32]> for Addresses {
    fn from(registers: &[u32]) -> Self {
        let (addresses, low_memory_address): (Vec<Address>, Option<Memory>) = registers
            .iter()
            .cloned()
            .fold((Vec::new(), None), move |(mut addresses, low_memory_address), register| match low_memory_address {
                Some(low_memory_address) => {
                    assert!(!low_memory_address.memory_space_indicator());
                    assert!(matches!(low_memory_address.size(), Size::Bits64));
                    let prefetchable: bool = low_memory_address.prefetchable();
                    let low_address: u32 = low_memory_address.base_address() << Memory::BASE_ADDRESS_OFFSET;
                    let low_address: u64 = low_address as u64;
                    let high_address: u32 = register;
                    let high_address: u64 = high_address as u64;
                    let address: u64 = low_address | (high_address << u32::BITS);
                    let address = Address::Memory {
                        address,
                        prefetchable,
                    };
                    addresses.push(address);
                    (addresses, None)
                },
                None => {
                    let memory: Memory = register.into();
                    let memory: Option<Memory> = (!memory.memory_space_indicator()).then_some(memory);
                    let io: Io = register.into();
                    let io: Option<Io> = io.io_space_indicator().then_some(io);
                    match (memory, io) {
                        (Some(memory), None) => match memory.size() {
                            Size::Bits32 => {
                                let address: u32 = memory.base_address() << Memory::BASE_ADDRESS_OFFSET;
                                let address: u64 = address as u64;
                                let prefetchable: bool = memory.prefetchable();
                                let address = Address::Memory {
                                    address,
                                    prefetchable,
                                };
                                addresses.push(address);
                                (addresses, None)
                            },
                            Size::Bits64 => (addresses, Some(memory)),
                        },
                        (None, Some(io)) => {
                            let address: u32 = io.base_address() << Io::BASE_ADDRESS_OFFSET;
                            let address = Address::Io {
                                address,
                            };
                            addresses.push(address);
                            (addresses, None)
                        },
                        _ => unreachable!(),
                    }
                },
            });
        assert!(matches!(low_memory_address, None));
        Self(addresses)
    }
}

impl fmt::Debug for Addresses {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(addresses) = self;
        formatter
            .debug_list()
            .entries(addresses.iter())
            .finish()
    }
}

/// # Base Address
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h)
#[derive(Debug)]
pub enum Address {
    Io {
        address: u32,
    },
    Memory {
        address: u64,
        prefetchable: bool,
    },
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

