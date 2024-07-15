use {
    core::{
        fmt,
        slice,
    },
    super::{
        EXT_OP_PREFIX,
        NameSpaceModifierObj,
        NamedObj,
        OP_REGION_OP,
        SCOPE_OP,
    },
};

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Object {
    NameSpaceModifierObj(NameSpaceModifierObj),
    NamedObj(NamedObj),
}

impl Object {
    pub fn length(&self) -> usize {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => name_space_modifier_obj.length(),
            Self::NamedObj(named_obj) => unimplemented!(),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => formatter
                .debug_struct("Object")
                .field("name_space_modifier_obj", name_space_modifier_obj)
                .finish(),
            Self::NamedObj(named_obj) => formatter
                .debug_struct("Object")
                .field("named_obj", named_obj)
                .finish(),
        }
    }
}

impl From<&[u8]> for Object {
    fn from(aml: &[u8]) -> Self {
        let mut aml_iterator: slice::Iter<u8> = aml.iter();
        match *aml_iterator.next().unwrap() {
            EXT_OP_PREFIX => match *aml_iterator.next().unwrap() {
                OP_REGION_OP => {
                    let named_obj: NamedObj = aml.into();
                    Self::NamedObj(named_obj)
                },
                unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
            }
            SCOPE_OP => {
                let name_space_modifier_obj: NameSpaceModifierObj = aml.into();
                Self::NameSpaceModifierObj(name_space_modifier_obj)
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

