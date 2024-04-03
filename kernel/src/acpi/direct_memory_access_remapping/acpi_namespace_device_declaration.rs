use core::{
    fmt,
    mem,
    slice,
    str,
};

/// # ACPI Name-space Device Declaration Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.7 ACPI Name-space Device Declaration Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
    reserved0: [u8; 3],
    acpi_device_number: u8,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn acpi_object_name(&self) -> &str {
        let structure: *const Self = self as *const Self;
        let acpi_object_name: *const Self = unsafe {
            structure.add(1)
        };
        let acpi_object_name: *const u8 = acpi_object_name as *const u8;
        let length: usize = self.length() - mem::size_of::<Self>();
        let acpi_object_name: &[u8] = unsafe {
            slice::from_raw_parts(acpi_object_name, length)
        };
        str::from_utf8(acpi_object_name).unwrap()
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u16 = self.structure_type;
        let length: u16 = self.length;
        let acpi_device_number: u8 = self.acpi_device_number;
        let acpi_object_name: &str = self.acpi_object_name();
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("acpi_device_number", &acpi_device_number)
            .field("acpi_object_name", &acpi_object_name)
            .finish()
    }
}

