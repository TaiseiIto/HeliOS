use {
    core::fmt,
    super::{
        DeviceOp,
        Reader,
        PkgLength,
        NameString,
        TermList,
    },
};

/// # DefDevice
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefDevice {
    device_op: DeviceOp,
    pkg_length: PkgLength,
    name_string: NameString,
    term_list: TermList,
}

impl fmt::Debug for DefDevice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            device_op,
            pkg_length,
            name_string,
            term_list,
        } = self;
        formatter
            .debug_tuple("DefDevice")
            .field(device_op)
            .field(pkg_length)
            .field(name_string)
            .field(term_list)
            .finish()
    }
}

impl From<&[u8]> for DefDevice {
    fn from(aml: &[u8]) -> Self {
        let (device_op, aml): (DeviceOp, &[u8]) = DeviceOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        let (term_list, aml): (TermList, &[u8]) = TermList::read(aml);
        Self {
            device_op,
            pkg_length,
            name_string,
            term_list,
        }
    }
}

impl Reader<'_> for DefDevice {
    fn length(&self) -> usize {
        let Self {
            device_op,
            pkg_length,
            name_string,
            term_list
        } = self;
        device_op.length()
        + pkg_length.length()
        + name_string.length()
        + term_list.length()
    }

    fn matches(aml: &[u8]) -> bool {
        DeviceOp::matches(aml)
    }
}

