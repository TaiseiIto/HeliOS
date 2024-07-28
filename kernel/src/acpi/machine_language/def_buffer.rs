use {
    core::fmt,
    super::{
        BufferOp,
        PkgLength,
        Reader,
    },
};

/// # DefBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefBuffer {
    buffer_op: BufferOp,
    pkg_length: PkgLength,
}

impl fmt::Debug for DefBuffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            buffer_op,
            pkg_length,
        } = self;
        formatter
            .debug_tuple("DefBuffer")
            .field(buffer_op)
            .field(pkg_length)
            .finish()
    }
}

impl From<&[u8]> for DefBuffer {
    fn from(aml: &[u8]) -> Self {
        let (buffer_op, aml) = BufferOp::read(aml);
        let (pkg_length, aml) = PkgLength::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefBuffer {
    fn length(&self) -> usize {
        let Self {
            buffer_op,
            pkg_length,
        } = self;
        buffer_op.length()
        + pkg_length.length()
    }

    fn matches(aml: &[u8]) -> bool {
        BufferOp::matches(aml)
    }
}

