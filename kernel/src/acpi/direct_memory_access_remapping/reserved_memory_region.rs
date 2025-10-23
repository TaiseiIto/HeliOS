use {
    super::hardware_unit_definition,
    alloc::vec::Vec,
    core::{fmt, mem::size_of, slice},
};

/// # Reserved Memory Region Reporting Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.4 Reserved Memory Region Reporting Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
    __: u16,
    segment_number: u16,
    base_address: u64,
    limit_address: u64,
}

impl Structure {
    pub fn bytes(&self) -> &[u8] {
        let structure: *const Self = self as *const Self;
        let first_byte: *const Self = unsafe { structure.add(1) };
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.length() - size_of::<Self>();
        unsafe { slice::from_raw_parts(first_byte, size) }
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    fn iter(&self) -> hardware_unit_definition::Scopes<'_> {
        self.into()
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u16 = self.structure_type;
        let length: u16 = self.length;
        let segment_number: u16 = self.segment_number;
        let base_address: u64 = self.base_address;
        let limit_address: u64 = self.limit_address;
        let scopes: Vec<&hardware_unit_definition::scope::Structure> = self.iter().collect();
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("segment_number", &segment_number)
            .field("base_address", &base_address)
            .field("limit_address", &limit_address)
            .field("scopes", &scopes)
            .finish()
    }
}
