use crate::x64;

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
                unsafe {
                    *address
                }
            },
            Self::Port(port) => x64::port::inb(*port),
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
            },
            Self::Port(port) => x64::port::outb(*port, value),
        }
    }
}

