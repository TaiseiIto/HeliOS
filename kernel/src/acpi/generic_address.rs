use crate::x64::port;

/// # Generic Address Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.3.2 Generic Address Structure
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    address_space_id: u8,
    #[allow(dead_code)]
    register_bit_width: u8,
    #[allow(dead_code)]
    register_bit_offset: u8,
    #[allow(dead_code)]
    access_size: u8,
    address: u64,
}

impl Structure {
    pub fn access_size(&self) -> usize {
        1 << (self.access_size - 1)
    }

    pub fn address(&self) -> usize {
        self.address as usize
    }

    pub fn get<T>(&self) -> &T {
        let address_space_id: SpaceId = self.address_space_id.into();
        assert_eq!(address_space_id, SpaceId::SystemMemorySpace);
        let address: usize = self.address as usize;
        let address: *const T = address as *const T;
        unsafe {
            &*address
        }
    }

    pub fn get_mut<T>(&mut self) -> &mut T {
        let address_space_id: SpaceId = self.address_space_id.into();
        assert_eq!(address_space_id, SpaceId::SystemMemorySpace);
        let address: usize = self.address as usize;
        let address: *mut T = address as *mut T;
        unsafe {
            &mut *address
        }
    }

    pub fn is_null(&self) -> bool {
        self.address == 0
    }

    pub fn read_byte(&self) -> u8 {
        assert_eq!(self.access_size(), 1);
        match self.address_space_id.into() {
            SpaceId::SystemMemorySpace => {
                let address: usize = self.address as usize;
                let address: *const u8 = address as *const u8;
                let address: &u8 = unsafe {
                    &*address
                };
                *address
            },
            SpaceId::SystemIoSpace => {
                let port: u16 = self.address as u16;
                port::inb(port)
            },
            _ => unimplemented!(),
        }
    }

    pub fn read_word(&self) -> u16 {
        assert_eq!(self.access_size(), 2);
        match self.address_space_id.into() {
            SpaceId::SystemMemorySpace => {
                let address: usize = self.address as usize;
                let address: *const u16 = address as *const u16;
                let address: &u16 = unsafe {
                    &*address
                };
                *address
            },
            SpaceId::SystemIoSpace => {
                let port: u16 = self.address as u16;
                port::inw(port)
            },
            _ => unimplemented!(),
        }
    }

    pub fn read_dword(&self) -> u32 {
        assert_eq!(self.access_size(), 4);
        match self.address_space_id.into() {
            SpaceId::SystemMemorySpace => {
                let address: usize = self.address as usize;
                let address: *const u32 = address as *const u32;
                let address: &u32 = unsafe {
                    &*address
                };
                *address
            },
            SpaceId::SystemIoSpace => {
                let port: u16 = self.address as u16;
                port::inl(port)
            },
            _ => unimplemented!(),
        }
    }

    pub fn read_qword(&self) -> u64 {
        assert_eq!(self.access_size(), 8);
        match self.address_space_id.into() {
            SpaceId::SystemMemorySpace => {
                let address: usize = self.address as usize;
                let address: *const u64 = address as *const u64;
                let address: &u64 = unsafe {
                    &*address
                };
                *address
            },
            SpaceId::SystemIoSpace => {
                let low_port: u16 = self.address as u16;
                let low: u64 = port::inl(low_port) as u64;
                let high_port: u16 = low_port + 1;
                let high: u64 = port::inl(high_port) as u64;
                low + (high << u32::BITS)
            },
            _ => unimplemented!(),
        }
    }
}

/// # Address Space ID
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.3.2 Generic Address Structure
#[derive(Debug, Eq, PartialEq)]
pub enum SpaceId {
    SystemMemorySpace,
    SystemIoSpace,
    PciConfigurationSpace,
    EmbeddedController,
    SmBus,
    SystemCmos,
    PciBarTarget,
    Ipmi,
    GeneralPurposeIo,
    GenericSerialBus,
    PlatformCommunicationsChannel,
    PlatformRuntimeMechanism,
    Reserved(u8),
    FunctionalFixedHardware,
    OemDefined(u8),
}

impl From<u8> for SpaceId {
    fn from(space_id: u8) -> Self {
        match space_id {
            0x00 => Self::SystemMemorySpace,
            0x01 => Self::SystemIoSpace,
            0x02 => Self::PciConfigurationSpace,
            0x03 => Self::EmbeddedController,
            0x04 => Self::SmBus,
            0x05 => Self::SystemCmos,
            0x06 => Self::PciBarTarget,
            0x07 => Self::Ipmi,
            0x08 => Self::GeneralPurposeIo,
            0x09 => Self::GenericSerialBus,
            0x0a => Self::PlatformCommunicationsChannel,
            0x0b => Self::PlatformRuntimeMechanism,
            space_id @ 0x0c..=0x7e => Self::Reserved(space_id),
            0x7f => Self::FunctionalFixedHardware,
            space_id @ 0x80..=0xff => Self::OemDefined(space_id),
        }
    }
}

