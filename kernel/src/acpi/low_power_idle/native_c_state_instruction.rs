use {
    bitfield_struct::bitfield,
    super::super::generic_address,
};

/// # Native C-State instruction based LPI structure
/// ## References
/// * [Intel Low Power S0 Idle](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf) 2.2.1. Native C-State instruction based LPI structure type
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u32,
    length: u32,
    unique_id: u16,
    reserved0: u16,
    flags: Flags,
    entry_trigger: generic_address::Structure,
    residency: u32,
    latency: u32,
    residency_counter: generic_address::Structure,
    residency_counter_frequency: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # Flags Field
/// ## References
/// * [Intel Low Power S0 Idle](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf) 2.2.1.1. Flags Field
#[bitfield(u32)]
struct Flags {
    disabled: bool,
    counter_not_available: bool,
    #[bits(30, access = RO)]
    reserved0: u32,
}

