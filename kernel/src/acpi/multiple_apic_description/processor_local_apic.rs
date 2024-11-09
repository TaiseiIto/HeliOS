use bitfield_struct::bitfield;

/// # Processor Local APIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.2 Processor Local APIC Structure
#[derive(Clone, Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    acpi_processor_uid: u8,
    apic_id: u8,
    #[allow(dead_code)]
    flags: Flags,
}

impl Structure {
    pub fn apic_id(&self) -> u8 {
        self.apic_id
    }

    pub fn is_enabled(&self) -> bool {
        let flags: Flags = self.flags;
        flags.enabled()
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # Local APIC Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.2 Table 5.23 Local APIC Flags
#[bitfield(u32)]
pub struct Flags {
    enabled: bool,
    online_capable: bool,
    #[bits(30)]
    __: u32,
}

