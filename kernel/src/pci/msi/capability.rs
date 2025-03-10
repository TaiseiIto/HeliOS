use {
    core::fmt,
    super::super::Function,
};

pub mod msi_x;

/// # MSI Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.1. MSI Capability Structure
#[derive(Clone)]
#[repr(packed)]
pub struct Header {
    capability_id: u8,
    next_pointer: u8,
}

impl Header {
    pub fn capability_id(&self) -> u8 {
        self.capability_id
    }

    pub fn next_pointer(&self) -> u8 {
        self.next_pointer
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.capability_id().into() {
            Id::MsiX => {
                let structure: &msi_x::Structure = self.into();
                structure.fmt(formatter)
            },
            _ => unimplemented!(),
        }
    }
}

pub struct HeaderWithFunction<'a> {
    function: &'a Function,
    header: &'a Header,
}

impl<'a> HeaderWithFunction<'a> {
    pub fn new(header: &'a Header, function: &'a Function) -> Self {
        Self {
            function,
            header,
        }
    }
}

impl fmt::Debug for HeaderWithFunction<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            function,
            header,
        } = self;
        match header.capability_id().into() {
            Id::MsiX => {
                let structure: &msi_x::Structure = (*header).into();
                let structure_with_function = msi_x::StructureWithFunction::new(structure, function);
                structure_with_function.fmt(formatter)
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
pub struct Headers<'a> {
    function: &'a Function,
    next_pointer: u8,
}

impl fmt::Debug for Headers<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self
                .clone()
                .map(|header| HeaderWithFunction::new(header, self.function)))
            .finish()
    }
}

impl<'a> From<&'a Function> for Headers<'a> {
    fn from(function: &'a Function) -> Self {
        let next_pointer: u8 = function
            .header()
            .capabilities_pointer();
        Self {
            function,
            next_pointer,
        }
    }
}

impl<'a> Iterator for Headers<'a> {
    type Item = &'a Header;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            function,
            next_pointer,
        } = self;
        let function: *const Function = *function as *const Function;
        let function: usize = function as usize;
        let offset: u8 = *next_pointer;
        (offset != 0).then(|| {
            let offset: usize = offset as usize;
            let structure: usize = function + offset;
            let structure: *const Header = structure as *const Header;
            let structure: &Header = unsafe {
                &*structure
            };
            *next_pointer = structure.next_pointer;
            structure
        })
    }
}

/// # Capability IDs
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) H. Capability IDs
#[derive(Debug)]
pub enum Id {
    Reserved(u8),
    PciPowerManagementInterface,
    Agp,
    Vpd,
    SlotIdentification,
    Msi,
    CompactPciHotSwap,
    PciX,
    HyperTransport,
    VendorSpecific,
    DebugPort,
    CompactPciCentralResourceControl,
    PciHotPlug,
    PciBridgeSubsystemVendorId,
    Agp8x,
    SecureDevice,
    PciExpress,
    MsiX,
}

impl From<u8> for Id {
    fn from(id: u8) -> Self {
        match id {
            0x01 => Self::PciPowerManagementInterface,
            0x02 => Self::Agp,
            0x03 => Self::Vpd,
            0x04 => Self::SlotIdentification,
            0x05 => Self::Msi,
            0x06 => Self::CompactPciHotSwap,
            0x07 => Self::PciX,
            0x08 => Self::HyperTransport,
            0x09 => Self::VendorSpecific,
            0x0a => Self::DebugPort,
            0x0b => Self::CompactPciCentralResourceControl,
            0x0c => Self::PciHotPlug,
            0x0d => Self::PciBridgeSubsystemVendorId,
            0x0e => Self::Agp8x,
            0x0f => Self::SecureDevice,
            0x10 => Self::PciExpress,
            0x11 => Self::MsiX,
            id => Self::Reserved(id),
        }
    }
}

