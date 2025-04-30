use {
    alloc::{
        collections::btree_map::BTreeMap,
        vec::Vec,
    },
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        slice,
    },
    crate::x64,
};

/// # Index to Address
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.1 Base Address Registers (Offset 10h - 24h)
pub struct Index2Address(BTreeMap<usize, Address>);

impl Index2Address {
    pub fn get(&self, index: usize) -> Option<&Address> {
        let Self(index2address) = self;
        index2address.get(&index)
    }
}

impl From<&[u32]> for Index2Address {
    fn from(registers: &[u32]) -> Self {
        let (index2address, index_and_low_memory_address): (BTreeMap<usize, Address>, Option<(usize, Memory)>) = registers
            .iter()
            .cloned()
            .enumerate()
            .fold((BTreeMap::new(), None), move |(mut index2address, index_and_low_memory_address), (index, register)| match index_and_low_memory_address {
                Some((index, low_memory_address)) => {
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
                    index2address.insert(index, address);
                    (index2address, None)
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
                                index2address.insert(index, address);
                                (index2address, None)
                            },
                            Size::Bits64 => (index2address, Some((index, memory))),
                        },
                        (None, Some(io)) => {
                            let address: u32 = io.base_address() << Io::BASE_ADDRESS_OFFSET;
                            let address = Address::Io {
                                address,
                            };
                            index2address.insert(index, address);
                            (index2address, None)
                        },
                        _ => unreachable!(),
                    }
                },
            });
        assert!(index_and_low_memory_address.is_none());
        Self(index2address)
    }
}

impl fmt::Debug for Index2Address {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(index2address) = self;
        formatter
            .debug_map()
            .entries(index2address.iter())
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

impl Address {
    pub fn offset(&self, offset: usize) -> Self {
        match self {
            Self::Io {
                address,
            } => {
                let offset: u32 = offset as u32;
                let address: u32 = address + offset;
                Self::Io {
                    address,
                }
            },
            Self::Memory {
                address,
                prefetchable,
            } => {
                let offset: u64 = offset as u64;
                let address: u64 = address + offset;
                let prefetchable: bool = *prefetchable;
                Self::Memory {
                    address,
                    prefetchable,
                }
            },
        }
    }

    pub fn read<T>(&self) -> T where T: Default {
        let mut read = T::default();
        let writer: &mut T = &mut read;
        let writer: *mut T = writer as *mut T;
        let writer: *mut u8 = writer as *mut u8;
        let size: usize = mem::size_of::<T>();
        let writer: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(writer, size)
        };
        writer
            .iter_mut()
            .enumerate()
            .for_each(|(index, byte)| match self {
                Self::Io {
                    address,
                } => {
                    let address: u16 = *address as u16;
                    let index: u16 = index as u16;
                    *byte = x64::port::inb(address + index);
                },
                Self::Memory {
                    address,
                    prefetchable: _,
                } => {
                    let address: usize = *address as usize;
                    let address: usize = address + index;
                    let address: *const u8 = address as *const u8;
                    let address: &u8 = unsafe {
                        &*address
                    };
                    *byte = *address;
                },
            });
        read
    }

    pub fn read_vector<T>(&self, length: usize) -> Vec<T> where T: Default {
        let size: usize = mem::size_of::<T>();
        (0..length)
            .map(|index| self
                .offset(index * size)
                .read())
            .collect()
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

