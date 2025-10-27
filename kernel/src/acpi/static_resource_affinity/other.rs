use core::{fmt, mem, slice};

#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }

    fn bytes(&self) -> &[u8] {
        let structure: *const Self = self as *const Self;
        let structure: usize = structure as usize;
        let first_byte: usize = structure + mem::size_of_val(self);
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = (self.length as usize) - mem::size_of_val(self);
        unsafe { slice::from_raw_parts(first_byte, size) }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u8 = self.structure_type;
        let length: u8 = self.length;
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("bytes", &self.bytes())
            .finish()
    }
}
