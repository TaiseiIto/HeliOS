use bitfield_struct::bitfield;

/// # PCI Express Slot Control Register
/// ## References
/// * [PCI_EXPRESS_SLOT_CONTROL_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_slot_control_register)
#[bitfield(u16)]
pub struct Register {
    attension_button_enable: bool,
    power_fault_detect_enable: bool,
    mrl_sensor_enable: bool,
    presence_detect_enable: bool,
    command_completed_enable: bool,
    hot_plug_interrupt_enable: bool,
    #[bits(2)]
    attention_indicator_control: u8,
    #[bits(2)]
    power_indicator_control: u8,
    power_controller_control: bool,
    electromechanical_lock_control: bool,
    data_link_state_change_enable: bool,
    #[bits(3)]
    __: u8,
}
