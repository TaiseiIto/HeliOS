use bitfield_struct::bitfield;

/// # PCI Express Device Capabilities 2 Register - 0x0A4
/// ## Referneces
/// * [PCI Express Device Capabilities 2 Register - 0x0A4](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 61. PCI Express Device Capabilities 2 Register - 0x0A4
/// * [ntddk.h](https://codemachine.com/downloads/win10.1511/ntddk.h)
#[bitfield(u32)]
pub struct Register {
    #[bits(4)]
    completion_timeout_ranges: u8,
    completion_timeout_disable_supported: bool,
    ari_forwarding_supported: bool,
    atomic_op_routing_supported: bool,
    atomic_op_completer_supported_32bit: bool,
    atomic_op_completer_supported_64bit: bool,
    cas_completer_supported_128bit: bool,
    no_ro_enabled_prpr_passing: bool,
    ltr_mechanism_supported: bool,
    #[bits(2)]
    tph_completer_supported: u8,
    #[bits(4)]
    __: u8,
    #[bits(2)]
    obff_supported: u8,
    extended_fmt_field_suported: bool,
    end_end_tlp_prefix_supported: bool,
    #[bits(2)]
    max_end_end_tlp_prefixes: u8,
    __: u8,
}
