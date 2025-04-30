use bitfield_struct::bitfield;

/// # PCI Expres Slot Status Register
/// ## References
/// * [PCI_EXPRESS_SLOT_STATUS_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_slot_status_register)
#[bitfield(u16)]
pub struct Register {
    attention_button_pressed: bool,
    power_fualt_detected: bool,
    mrl_sensor_changed: bool,
    presence_detect_changed: bool,
    command_completed: bool,
    mrl_sensor_state: bool,
    presence_detect_state: bool,
    electromechanical_lock_engaged: bool,
    data_link_state_changed: bool,
    #[bits(7)]
    __: u8,
}

