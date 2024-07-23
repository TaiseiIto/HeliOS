use {
    core::fmt,
    super::{
        ARG_OBJ_MAX,
        ARG_OBJ_MIN,
        LOCAL_OBJ_MAX,
        LOCAL_OBJ_MIN,
        Reader,
        SimpleName,
    },
};

/// # SuperName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum SuperName {
    SimpleName(SimpleName),
}

impl fmt::Debug for SuperName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SimpleName(simple_name) => formatter
                .debug_tuple("SuperName")
                .field(simple_name)
                .finish(),
        }
    }
}

impl From<&[u8]> for SuperName {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ARG_OBJ_MIN..=ARG_OBJ_MAX | LOCAL_OBJ_MIN..=LOCAL_OBJ_MAX => Self::SimpleName(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for SuperName {
    fn length(&self) -> usize {
        match self {
            Self::SimpleName(simple_name) => simple_name.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

