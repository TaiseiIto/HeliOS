use {
    alloc::string::String,
    core::fmt,
    crate::{
        com2_print,
        com2_println,
    },
    super::{
        NameString,
        OpRegionOp,
    },
};

/// # DefOpRegion
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefOpRegion {
    op_region_op: OpRegionOp,
    name_string: NameString,
}

impl fmt::Debug for DefOpRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            op_region_op,
            name_string,
        } = self;
        formatter
            .debug_tuple("DefOpRegion")
            .field(op_region_op)
            .field(name_string)
            .finish()
    }
}

impl From<&[u8]> for DefOpRegion {
    fn from(aml: &[u8]) -> Self {
        let op_region_op: OpRegionOp = aml.into();
        let aml: &[u8] = &aml[op_region_op.length()..];
        let name_string: NameString = aml.into();
        let name: String = (&name_string).into();
        com2_println!("name = {:#x?}", name);
        let aml: &[u8] = &aml[name_string.length()..];
        unimplemented!()
    }
}

