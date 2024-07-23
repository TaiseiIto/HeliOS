use super::Reader;

pub const LOCAL_OBJ_MIN: u8 = 0x60;
pub const LOCAL_OBJ_MAX: u8 = 0x67;

/// # Local Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.2 Local Objects Encoding
#[derive(Debug)]
pub struct LocalObj(u8);

impl From<&[u8]> for LocalObj {
    fn from(aml: &[u8]) -> Self {
        let local_obj: u8 = *aml.first().unwrap();
        assert!((LOCAL_OBJ_MIN..=LOCAL_OBJ_MAX).contains(&local_obj));
        let local_obj: u8 = local_obj - LOCAL_OBJ_MIN;
        Self(local_obj)
    }
}

impl Reader<'_> for LocalObj {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| (LOCAL_OBJ_MIN..=LOCAL_OBJ_MAX).contains(head))
    }
}

