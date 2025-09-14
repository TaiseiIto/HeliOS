use crate::{acpi, x64};

pub enum Mapped {
    Memory(usize),
    Port(u16),
}

impl Mapped {
    pub fn memory(address: usize) -> Self {
        Self::Memory(address)
    }

    pub fn port(port: u16) -> Self {
        Self::Port(port)
    }

    pub fn read_u8(&self) -> u8 {
        match self {
            Self::Memory(address) => {
                let address: usize = *address;
                let address: *const u8 = address as *const u8;
                unsafe { *address }
            }
            Self::Port(port) => x64::port::inb(*port),
        }
    }

    pub fn read_u16(&self) -> u16 {
        match self {
            Self::Memory(address) => {
                let address: usize = *address;
                let address: *const u16 = address as *const u16;
                unsafe { *address }
            }
            Self::Port(port) => x64::port::inw(*port),
        }
    }

    pub fn read_u32(&self) -> u32 {
        match self {
            Self::Memory(address) => {
                let address: usize = *address;
                let address: *const u32 = address as *const u32;
                unsafe { *address }
            }
            Self::Port(port) => x64::port::inl(*port),
        }
    }

    pub fn write_u8(&mut self, value: u8) {
        match self {
            Self::Memory(address) => {
                let address: usize = *address;
                let address: *mut u8 = address as *mut u8;
                unsafe {
                    *address = value;
                }
            }
            Self::Port(port) => x64::port::outb(*port, value),
        }
    }

    pub fn write_u16(&mut self, value: u16) {
        match self {
            Self::Memory(address) => {
                let address: usize = *address;
                let address: *mut u16 = address as *mut u16;
                unsafe {
                    *address = value;
                }
            }
            Self::Port(port) => x64::port::outw(*port, value),
        }
    }

    pub fn write_u32(&mut self, value: u32) {
        match self {
            Self::Memory(address) => {
                let address: usize = *address;
                let address: *mut u32 = address as *mut u32;
                unsafe {
                    *address = value;
                }
            }
            Self::Port(port) => x64::port::outl(*port, value),
        }
    }
}

impl From<&acpi::generic_address::Structure> for Mapped {
    fn from(address: &acpi::generic_address::Structure) -> Self {
        let address: usize = address.address();
        Self::Memory(address)
    }
}
