use {
    core::ops::RangeInclusive,
    super::Reader,
};

pub const ARG_OBJ_MIN: u8 = 0x68;
pub const ARG_OBJ_MAX: u8 = 0x6e;

/// # Arg Objects Encoding
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.6.1 Arg Objects Encoding
#[derive(Debug)]
pub struct ArgObj(u8);

impl From<&[u8]> for ArgObj {
    fn from(aml: &[u8]) -> Self {
        let arg_obj: u8 = *aml.first().unwrap();
        assert!((ARG_OBJ_MIN..=ARG_OBJ_MAX).contains(&arg_obj));
        let arg_obj: u8 = arg_obj - ARG_OBJ_MIN;
        Self(arg_obj)
    }
}

impl Reader<'_> for ArgObj {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

