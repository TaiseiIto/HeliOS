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
    #[allow(dead_code)]
    structure_type: u32,
    length: u32,
    #[allow(dead_code)]
    unique_id: u16,
    __: u16,
    #[allow(dead_code)]
    flags: Flags,
    #[allow(dead_code)]
    entry_trigger: generic_address::Structure,
    #[allow(dead_code)]
    residency: u32,
    #[allow(dead_code)]
    latency: u32,
    #[allow(dead_code)]
    residency_counter: generic_address::Structure,
    #[allow(dead_code)]
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
    #[bits(30)]
    __: u32,
}

