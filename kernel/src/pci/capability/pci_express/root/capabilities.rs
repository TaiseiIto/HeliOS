use bitfield_struct::bitfield;

/// # PCI Express Root Capabilities Register
/// ## References
/// * [PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_root_capabilities_register)
#[bitfield(u16)]
pub struct Register {
    crs_software_visibility: bool,
    #[bits(15)]
    __: u16,
}

