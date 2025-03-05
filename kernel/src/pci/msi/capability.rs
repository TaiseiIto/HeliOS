use {
    core::fmt,
    super::super::Function,
};

/// # MSI Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.1. MSI Capability Structure
#[repr(packed)]
pub struct Structure {
    capability_id: u8,
    next_pointer: u8,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            capability_id,
            next_pointer,
        } = self;
        let capability_id: Id = (*capability_id).into();
        formatter
            .debug_struct("Structure")
            .field("capability_id", &capability_id)
            .field("next_poitner", next_pointer)
            .finish()
    }
}

pub struct Structures<'a> {
    function: &'a Function,
    next_pointer: u8,
}

impl<'a> From<&'a Function> for Structures<'a> {
    fn from(function: &'a Function) -> Self {
        let next_pointer: u8 = function.capabilities_pointer();
        Self {
            function,
            next_pointer,
        }
    }
}

impl<'a> Iterator for Structures<'a> {
    type Item = &'a Structure;

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
            let structure: *const Structure = structure as *const Structure;
            let structure: &Structure = unsafe {
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

