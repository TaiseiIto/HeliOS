use {
    core::fmt,
    super::{
        NameSeg,
        PkgLength,
        Reader,
    },
};

/// # NamedField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct NamedField {
    name_seg: NameSeg,
    pkg_length: PkgLength,
}

impl fmt::Debug for NamedField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name_seg,
            pkg_length,
        } = self;
        formatter
            .debug_tuple("NamedField")
            .field(name_seg)
            .field(pkg_length)
            .finish()
    }
}

impl From<&[u8]> for NamedField {
    fn from(aml: &[u8]) -> Self {
        let (name_seg, aml): (NameSeg, &[u8]) = NameSeg::read(aml);
        let pkg_length: PkgLength = aml.into();
        Self {
            name_seg,
            pkg_length,
        }
    }
}

impl Reader<'_> for NamedField {
    fn length(&self) -> usize {
        let Self {
            name_seg,
            pkg_length,
        } = self;
        name_seg.length() + pkg_length.length()
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

