use super::Reader;

const OP_REGION_OP_SUFFIX: u8 = 0x80;

/// # OpRegionOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct OpRegionOpSuffix;

impl From<&[u8]> for OpRegionOpSuffix {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self
    }
}

impl Reader<'_> for OpRegionOpSuffix {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| *head == OP_REGION_OP_SUFFIX)
    }
}

