use bitfield_struct::bitfield;

/// # PCI Express Root Status Register
/// ## References
/// * [PCI_EXPRESS_ROOT_STATUS_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_root_status_register)
#[bitfield(u32)]
pub struct Register {
    pme_requestor_id: u16,
    pme_status: bool,
    pme_pending: bool,
    #[bits(14)]
    __: u16,
}

