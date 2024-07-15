use super::OpRegionOp;

/// # DefOpRegion
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct DefOpRegion {
    op_region_op: OpRegionOp,
}

impl From<&[u8]> for DefOpRegion {
    fn from(aml: &[u8]) -> Self {
        let op_region_op: OpRegionOp = aml.into();
        let aml: &[u8] = &aml[op_region_op.length()..];
        unimplemented!()
    }
}

