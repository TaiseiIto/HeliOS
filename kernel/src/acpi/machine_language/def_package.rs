use {
    core::fmt,
    super::{
        PackageOp,
        PkgLength,
        Reader,
    },
};

/// # DefPackage
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefPackage {
    package_op: PackageOp,
    pkg_length: PkgLength,
}

impl fmt::Debug for DefPackage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            package_op,
            pkg_length,
        } = self;
        formatter
            .debug_tuple("DefPackage")
            .field(package_op)
            .field(pkg_length)
            .finish()
    }
}

impl From<&[u8]> for DefPackage {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (package_op, aml): (PackageOp, &[u8]) = PackageOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefPackage {
    fn length(&self) -> usize {
        let Self {
            package_op,
            pkg_length,
        } = self;
        package_op.length()
        + pkg_length.length()
    }

    fn matches(aml: &[u8]) -> bool {
        PackageOp::matches(aml)
    }
}

