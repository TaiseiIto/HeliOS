use {
    core::fmt,
    super::{
        ARG_OBJ_MAX,
        ARG_OBJ_MIN,
        LOCAL_OBJ_MAX,
        LOCAL_OBJ_MIN,
        NULL_NAME,
        NullName,
        Reader,
        SuperName,
    },
};

/// # Target
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum Target {
    NullName(NullName),
    SuperName(SuperName),
}

impl fmt::Debug for Target {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullName(null_name) => formatter
                .debug_tuple("Target")
                .field(null_name)
                .finish(),
            Self::SuperName(super_name) => formatter
                .debug_tuple("Target")
                .field(super_name)
                .finish(),
        }
    }
}

impl From<&[u8]> for Target {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            NULL_NAME => Self::NullName(aml.into()),
            ARG_OBJ_MIN..=ARG_OBJ_MAX | LOCAL_OBJ_MIN..=LOCAL_OBJ_MAX => Self::SuperName(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for Target {
    fn length(&self) -> usize {
        match self {
            Self::NullName(null_name) => null_name.length(),
            Self::SuperName(super_name) => super_name.length(),
        }
    }
}

