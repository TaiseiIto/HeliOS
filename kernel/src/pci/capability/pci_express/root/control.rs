use bitfield_struct::bitfield;

/// # PCI Express Root Control Register
/// ## References
/// * [PCI_EXPRESS_ROOT_CONTROL_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_root_control_register)
#[bitfield(u16)]
pub struct Register {
    correctable_serr_enable: bool,
    non_fatal_serr_enable: bool,
    fatal_serr_enable: bool,
    pme_interrupt_enable: bool,
    crs_software_visibility_enable: bool,
    #[bits(11)]
    __: u16,
}

