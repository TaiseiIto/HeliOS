use bitfield_struct::bitfield;

/// # PCI Express Slot Capabilities Register
/// ## References
/// * [PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_slot_capabilities_register)
#[bitfield(u32)]
pub struct Register {
    attention_button_present: bool,
    power_controller_present: bool,
    mrl_sensor_present: bool,
    attention_indicator_present: bool,
    opwer_indicator_present: bool,
    hot_plug_surprise: bool,
    hot_plug_capable: bool,
    slot_power_limit: u8,
    #[bits(2)]
    slot_power_limit_scale: u8,
    electromechanical_lock_present: bool,
    no_command_completed_support: bool,
    #[bits(13)]
    physical_slot_number: u16,
}

