use {
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # Memory Mapped Configuration Table
/// ## References
/// * [PCI Express](https://wiki.osdev.org/PCI_Express)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    reserved0: u64,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn configuration_space_base_address_allocations(&self) -> &[ConfigurationSpaceBaseAddressAllocation] {
        let address: *const Self = self as *const Self;
        let address: *const Self = unsafe {
            address.add(1)
        };
        let address: *const ConfigurationSpaceBaseAddressAllocation = address as *const ConfigurationSpaceBaseAddressAllocation;
        let length: usize = (self.header.table_size() - mem::size_of::<Self>()) / mem::size_of::<ConfigurationSpaceBaseAddressAllocation>();
        unsafe {
            slice::from_raw_parts(address, length)
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("configuration_space_base_address_allocations", &self.configuration_space_base_address_allocations())
            .finish()
    }
}

/// # Configuration Space Base Address Allocation Structure
/// ## References
/// * [PCI Express](https://wiki.osdev.org/PCI_Express)
#[derive(Debug)]
#[repr(packed)]
struct ConfigurationSpaceBaseAddressAllocation {
    base_address: u64,
    pci_segment_group_number: u16,
    start_pci_bus_number: u8,
    end_pci_bus_number: u8,
    reserved0: u32,
}

