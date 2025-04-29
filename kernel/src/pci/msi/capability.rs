use {
    core::fmt,
    super::super::Function,
};

pub mod msi;
pub mod msi_x;
pub mod pci_power_management;
pub mod vendor_specific;

/// # MSI Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.1. MSI Capability Structure
#[derive(Clone, Debug)]
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

#[derive(Clone)]
pub struct Headers<'a> {
    function: &'a Function,
    next_pointer: u8,
}

impl Headers<'_> {
    fn next_pointer(&self) -> Option<u8> {
        let next_pointer: u8 = self.next_pointer;
        (next_pointer != 0).then_some(next_pointer)
    }

    fn next_header(&self) -> Option<&Header> {
        self.next_pointer()
            .map(|next_pointer| {
                let function: &Function = self.function;
                let function: *const Function = function as *const Function;
                let function: usize = function as usize;
                let next_pointer: usize = next_pointer as usize;
                let next_header: usize = function + next_pointer;
                let next_header: *const Header = next_header as *const Header;
                unsafe {
                    &*next_header
                }
            })
    }
}

impl fmt::Debug for Headers<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function: &Function = self.function;
        formatter
            .debug_list()
            .entries(self
                .clone()
                .map(|next_pointer| Structure::new(function, next_pointer)))
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
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_pointer()
            .zip(self
                .next_header()
                .cloned())
            .map(|(next_pointer, next_header)| {
                self.next_pointer = next_header.next_pointer;
                next_pointer
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

#[derive(Debug)]
pub enum Structure<'a> {
    Msi(&'a msi::Structure),
    MsiX(msi_x::StructureInFunction<'a>),
    PciBridgeSubsystemVendorId,
    PciPowerManagementInterface(&'a pci_power_management::Registers),
    Reserved(u8),
    VendorSpecific(vendor_specific::StructureInFunction<'a>),
}

impl<'a> Structure<'a> {
    fn new(function: &'a Function, next_pointer: u8) -> Self {
        let function_address: *const Function = function as *const Function;
        let function_address: usize = function_address as usize;
        let next_pointer_usize: usize = next_pointer as usize;
        let structure: usize = function_address + next_pointer_usize;
        let header: usize = structure;
        let header: *const Header = header as *const Header;
        let header: &Header = unsafe {
            &*header
        };
        match header.capability_id().into() {
            Id::Msi => {
                let structure: *const msi::Structure = structure as *const msi::Structure;
                let structure: &msi::Structure = unsafe {
                    &*structure
                };
                Self::Msi(structure)
            },
            Id::MsiX => Self::MsiX(msi_x::StructureInFunction::new(function, next_pointer)),
            Id::PciBridgeSubsystemVendorId => Self::PciBridgeSubsystemVendorId,
            Id::PciPowerManagementInterface => {
                let register: *const pci_power_management::Registers = structure as *const pci_power_management::Registers;
                let register: &pci_power_management::Registers = unsafe {
                    &*register
                };
                Self::PciPowerManagementInterface(register)
            },
            Id::Reserved(id) => Self::Reserved(id),
            Id::VendorSpecific => Self::VendorSpecific(vendor_specific::StructureInFunction::new(function, next_pointer)),
            id => unimplemented!("id = {:#x?}", id),
        }
    }
}

