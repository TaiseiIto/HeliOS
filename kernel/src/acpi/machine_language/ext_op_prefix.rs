use super::Reader;

pub const EXT_OP_PREFIX: u8 = 0x5b;

/// # ExtOpPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct ExtOpPrefix;

impl From<&[u8]> for ExtOpPrefix {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), EXT_OP_PREFIX);
        Self
    }
}

impl Reader<'_> for ExtOpPrefix {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| *head == EXT_OP_PREFIX)
    }
}

