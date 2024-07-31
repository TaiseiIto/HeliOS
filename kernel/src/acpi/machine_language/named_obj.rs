use {
    core::{
        fmt,
        slice,
    },
    super::{
        DefDevice,
        DefField,
        DefMethod,
        DefMutex,
        DefOpRegion,
        Reader,
    },
};

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub enum NamedObj {
    DefDevice(DefDevice),
    DefField(DefField),
    DefMethod(DefMethod),
    DefMutex(DefMutex),
    DefOpRegion(DefOpRegion),
}

impl fmt::Debug for NamedObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("NamedObj");
        match self {
            Self::DefDevice(def_device) => debug_tuple.field(def_device),
            Self::DefField(def_field) => debug_tuple.field(def_field),
            Self::DefMethod(def_method) => debug_tuple.field(def_method),
            Self::DefMutex(def_mutex) => debug_tuple.field(def_mutex),
            Self::DefOpRegion(def_op_region) => debug_tuple.field(def_op_region),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for NamedObj {
    fn from(aml: &[u8]) -> Self {
        if DefDevice::matches(aml) {
            Self::DefDevice(aml.into())
        } else if DefField::matches(aml) {
            Self::DefField(aml.into())
        } else if DefMethod::matches(aml) {
            Self::DefMethod(aml.into())
        } else if DefMutex::matches(aml) {
            Self::DefMutex(aml.into())
        } else if DefOpRegion::matches(aml) {
            Self::DefOpRegion(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for NamedObj {
    fn length(&self) -> usize {
        match self {
            Self::DefDevice(def_device) => def_device.length(),
            Self::DefField(def_field) => def_field.length(),
            Self::DefMethod(def_method) => def_method.length(),
            Self::DefMutex(def_mutex) => def_mutex.length(),
            Self::DefOpRegion(def_op_region) => def_op_region.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefDevice::matches(aml)
        || DefField::matches(aml)
        || DefMethod::matches(aml)
        || DefMutex::matches(aml)
        || DefOpRegion::matches(aml)
    }
}

