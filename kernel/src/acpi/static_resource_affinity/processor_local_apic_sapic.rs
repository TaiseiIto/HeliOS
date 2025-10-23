use {
    bitfield_struct::bitfield,
    core::{fmt, iter},
};

/// # Processor Local APIC/SAPIC Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.1 Processor Local APIC/SAPIC Affinity Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    proximity_domain0: u8,
    apic_id: u8,
    flags: Flags,
    local_sapic_eid: u8,
    proximity_domain1: [u8; 3],
    clock_domain: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn proximity_domain(&self) -> u32 {
        self.proximity_domain1
            .as_slice()
            .iter()
            .rev()
            .chain(iter::once(&self.proximity_domain0))
            .fold(0, |proximity_domain, byte| {
                (proximity_domain << u8::BITS) + (*byte as u32)
            })
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u8 = self.structure_type;
        let length: u8 = self.length;
        let proximity_domain: u32 = self.proximity_domain();
        let apic_id: u8 = self.apic_id;
        let flags: Flags = self.flags;
        let local_sapic_eid: u8 = self.local_sapic_eid;
        let clock_domain: u32 = self.clock_domain;
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("proximity_domain", &proximity_domain)
            .field("apic_id", &apic_id)
            .field("flags", &flags)
            .field("local_sapic_eid", &local_sapic_eid)
            .field("clock_domain", &clock_domain)
            .finish()
    }
}

/// # Processor Local APIC/SAPIC Affinity Structure Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.1 Processor Local APIC/SAPIC Affinity Structure
#[bitfield(u32)]
pub struct Flags {
    enabled: bool,
    #[bits(31)]
    __: u32,
}
