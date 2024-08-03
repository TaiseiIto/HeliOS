use super::Reader;

const DEVICE_OP_SUFFIX: u8 = 0x82;

/// # DeviceOpSuffix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct DeviceOpSuffix;

impl From<&[u8]> for DeviceOpSuffix {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self
    }
}

impl Reader<'_> for DeviceOpSuffix {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| *head == DEVICE_OP_SUFFIX)
    }
}

