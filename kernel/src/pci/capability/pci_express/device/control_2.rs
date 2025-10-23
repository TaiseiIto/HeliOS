use bitfield_struct::bitfield;

/// # PCI Express Device Control 2 and Status 2 Register - 0x0A8
/// ## Referneces
/// * [PCI Express Device Control 2 and Status 2 Register - 0x0A8](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 62. PCI Express Device Control 2 and Status 2 Register - 0x0A8
/// * [ntddk.h](https://codemachine.com/downloads/win10.1511/ntddk.h)
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    completion_timeout_value: u8,
    completion_timeout_disable: bool,
    ari_forwarding_enable: bool,
    atomic_operation_requester_enable: bool,
    atomic_op_egres_blocking: bool,
    ido_request_enable: bool,
    ido_completion_enable: bool,
    ltr_mechanism_enable: bool,
    #[bits(2)]
    rsvd: u8,
    #[bits(2)]
    obff_enable: u8,
    end_end_tlp_prefix_blocking: bool,
}
