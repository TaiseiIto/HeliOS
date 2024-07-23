use {
    core::fmt,
    super::{
        NameString,
        OpRegionOp,
        Reader,
        RegionLen,
        RegionOffset,
        RegionSpace,
    },
};

/// # DefOpRegion
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefOpRegion {
    op_region_op: OpRegionOp,
    name_string: NameString,
    region_space: RegionSpace,
    region_offset: RegionOffset,
    region_len: RegionLen,
}

impl fmt::Debug for DefOpRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            op_region_op,
            name_string,
            region_space,
            region_offset,
            region_len,
        } = self;
        formatter
            .debug_tuple("DefOpRegion")
            .field(op_region_op)
            .field(name_string)
            .field(region_space)
            .field(region_offset)
            .field(region_len)
            .finish()
    }
}

impl From<&[u8]> for DefOpRegion {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (op_region_op, aml): (OpRegionOp, &[u8]) = OpRegionOp::read(aml);
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        let (region_space, aml): (RegionSpace, &[u8]) = RegionSpace::read(aml);
        let (region_offset, aml): (RegionOffset, &[u8]) = RegionOffset::read(aml);
        let (region_len, aml): (RegionLen, &[u8]) = RegionLen::read(aml);
        Self {
            op_region_op,
            name_string,
            region_space,
            region_offset,
            region_len,
        }
    }
}

impl Reader<'_> for DefOpRegion {
    fn length(&self) -> usize {
        let Self {
            op_region_op,
            name_string,
            region_space,
            region_offset,
            region_len,
        } = self;
        op_region_op.length()
        + name_string.length()
        + region_space.length()
        + region_offset.length()
        + region_len.length()
    }

    fn matches(aml: &[u8]) -> bool {
        OpRegionOp::matches(aml)
    }
}

