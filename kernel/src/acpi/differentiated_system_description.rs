use {
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # DSDT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.9 Fixed ACPI Description Table (FADT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn definition_block(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let definition_block: usize = table + mem::size_of::<Self>();
        let definition_block: *const u8 = definition_block as *const u8;
        let definition_block_size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(definition_block, definition_block_size)
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .finish()
    }
}

