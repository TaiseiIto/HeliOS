use {
    alloc::vec::Vec,
    super::{
        ByteData,
        PkgLeadByte,
    },
};

/// # PkgLength
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[derive(Debug)]
pub struct PkgLength {
    pkg_lead_byte: PkgLeadByte,
    byte_data: Vec<ByteData>,
}

impl PkgLength {
    pub fn length(&self) -> usize {
        self.pkg_lead_byte.length() + self.byte_data
            .iter()
            .map(|byte_data| byte_data.length())
            .sum::<usize>()
    }

    pub fn pkg_length(&self) -> usize {
        (self.byte_data
            .iter()
            .rev()
            .fold(0, |length, byte_data| (length << u8::BITS) + byte_data.pkg_length()) << 4) + self.pkg_lead_byte.pkg_length()
    }
}

impl From<&[u8]> for PkgLength {
    fn from(bytes: &[u8]) -> Self {
        let pkg_lead_byte: PkgLeadByte = bytes.into();
        let bytes: &[u8] = &bytes[pkg_lead_byte.length()..];
        let (bytes, byte_data): (&[u8], Vec<ByteData>) = (0..pkg_lead_byte.byte_data_length())
            .fold((bytes, Vec::new()), |(bytes, mut byte_data), _| {
                let new_byte_data: ByteData = bytes.into();
                let bytes: &[u8] = &bytes[new_byte_data.length()..];
                byte_data.push(new_byte_data);
                (bytes, byte_data)
            });
        Self {
            pkg_lead_byte,
            byte_data,
        }
    }
}

