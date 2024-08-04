use super::{
    DefDevice,
    DefField,
    DefMethod,
    DefMutex,
    DefOpRegion,
};

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum NamedObj {
    DefDevice(DefDevice),
    DefField(DefField),
    DefMethod(DefMethod),
    DefMutex(DefMutex),
    DefOpRegion(DefOpRegion),
}

