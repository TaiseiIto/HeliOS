use super::{
    DualNamePrefix,
    NameSeg,
};

/// # DualNamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DualNamePath(
    DualNamePrefix,
    [NameSeg; 2],
);

