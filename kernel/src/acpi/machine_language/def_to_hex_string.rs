use {
    core::fmt,
    super::ToHexStringOp,
};

/// # DefToHexString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefToHexString {
    to_hex_string_op: ToHexStringOp,
}

impl DefToHexString {
    pub fn length(&self) -> usize {
        let Self {
            to_hex_string_op,
        } = self;
        to_hex_string_op.length()
    }
}

impl fmt::Debug for DefToHexString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            to_hex_string_op,
        } = self;
        formatter
            .debug_tuple("DefToHexString")
            .field(to_hex_string_op)
            .finish()
    }
}

impl From<&[u8]> for DefToHexString {
    fn from(aml: &[u8]) -> Self {
        let to_hex_string_op: ToHexStringOp = aml.into();
        let aml: &[u8] = &aml[to_hex_string_op.length()..];
        unimplemented!()
    }
}

