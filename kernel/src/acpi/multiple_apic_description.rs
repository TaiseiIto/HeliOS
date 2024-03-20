use {
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # MADT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12 Multiple APIC Description Table (MADT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    local_interrupt_controller_address: u32,
    flags: Flags,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_byte: usize = table + mem::size_of::<Self>();
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(first_byte, size)
        }
    }

    fn iter<'a>(&'a self) -> InterruptControllerStructures<'a> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let local_interrupt_controller_address: u32 = self.local_interrupt_controller_address;
        let flags: Flags = self.flags;
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("local_interrupt_controller_address", &local_interrupt_controller_address)
            .field("flags", &flags)
            .field("bytes", &self.iter())
            .finish()
    }
}

/// # Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12 Table 5.20 Multiple APIC Flags
#[bitfield(u32)]
struct Flags {
    pcat_compat: bool,
    #[bits(31, access = RO)]
    reserved0: u32,
}

#[derive(Debug)]
struct InterruptControllerStructures<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table> for InterruptControllerStructures<'a> {
    fn from(table: &'a Table) -> Self {
        let bytes: &[u8] = table.bytes();
        Self {
            bytes
        }
    }
}

