use {
    alloc::string::String,
    super::{
        DualNamePath,
        NameSeg,
        NullName,
    },
};

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum NamePath {
    DualNamePath(DualNamePath),
    NameSeg(NameSeg),
    NullName(NullName),
}

impl From<&NamePath> for String {
    fn from(name_path: &NamePath) -> Self {
        match name_path {
            NamePath::DualNamePath(dual_name_path) => dual_name_path.into(),
            NamePath::NameSeg(name_seg) => name_seg.into(),
            NamePath::NullName(null_name) => Self::new(),
        }
    }
}

