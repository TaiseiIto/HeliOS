use {
    core::{
        fmt,
        mem,
        slice,
        str,
    },
    super::processor_local_apic,
};

/// # I/O SAPIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.10 I/O SAPIC Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    acpi_processor_id: u8,
    local_sapic_id: u8,
    local_sapic_eid: u8,
    reserved0: [u8; 3],
    flags: processor_local_apic::Flags,
    acpi_processor_uid_value: u32,
}

impl Structure {
    fn acpi_processor_uid_string(&self) -> &str {
        let structure: *const Self = self as *const Self;
        let structure: usize = structure as usize;
        let acpi_processor_uid_string = structure + mem::size_of::<Self>();
        let acpi_processor_uid_string: *const u8 = acpi_processor_uid_string as *const u8;
        let acpi_processor_uid_string_length: usize = (self.length as usize) - mem::size_of::<Self>();
        let acpi_processor_uid_string: &[u8] = unsafe {
            slice::from_raw_parts(acpi_processor_uid_string, acpi_processor_uid_string_length)
        };
        let acpi_processor_uid_string: &[u8] = &acpi_processor_uid_string[..acpi_processor_uid_string.len() - 1];
        str::from_utf8(acpi_processor_uid_string).unwrap()
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags: processor_local_apic::Flags = self.flags;
        let acpi_processor_uid_value: u32 = self.acpi_processor_uid_value;
        formatter
            .debug_struct("Structure")
            .field("structure_type", &self.structure_type)
            .field("length", &self.length)
            .field("acpi_processor_id", &self.acpi_processor_id)
            .field("local_sapic_id", &self.local_sapic_id)
            .field("local_sapic_eid", &self.local_sapic_eid)
            .field("reserved0", &self.reserved0)
            .field("flags", &flags)
            .field("acpi_processor_uid_value", &acpi_processor_uid_value)
            .field("acpi_processor_uid_string", &self.acpi_processor_uid_string())
            .finish()
    }
}

