use {
    core::fmt,
    super::{
        DeviceOpSuffix,
        ExtOpPrefix,
        Reader,
    },
};

/// # DeviceOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DeviceOp {
    ext_op_prefix: ExtOpPrefix,
    device_op_suffix: DeviceOpSuffix,
}

impl fmt::Debug for DeviceOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
            device_op_suffix,
        } = self;
        formatter
            .debug_tuple("DeviceOp")
            .field(ext_op_prefix)
            .field(device_op_suffix)
            .finish()
    }
}

impl From<&[u8]> for DeviceOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        let (device_op_suffix, aml): (DeviceOpSuffix, &[u8]) = DeviceOpSuffix::read(aml);
        Self {
            ext_op_prefix,
            device_op_suffix,
        }
    }
}

impl Reader<'_> for DeviceOp {
    fn length(&self) -> usize {
        let Self {
            ext_op_prefix,
            device_op_suffix,
        } = self;
        ext_op_prefix.length() + device_op_suffix.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ExtOpPrefix::matches(aml) && {
            let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            DeviceOpSuffix::matches(aml)
        }
    }
}

