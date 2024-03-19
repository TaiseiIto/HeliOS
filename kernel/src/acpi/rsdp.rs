use core::{
    fmt,
    str,
};

/// # RSDP
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.5.3 Root System Description Pointer (RSDP) Structure
#[repr(packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

impl Rsdp {
    pub fn is_correct(&self) -> bool {
        let rsdp: *const Self = self as *const Self;
        let rsdp: *const [u8; 20] = rsdp as *const [u8; 20];
        let rsdp: &[u8; 20] = unsafe {
            &*rsdp
        };
        rsdp.iter()
            .fold(0x00u8, |sum, byte| sum.wrapping_add(*byte)) == 0
    }

    fn oemid(&self) -> &str {
        str::from_utf8(self.oemid.as_slice()).unwrap()
    }

    fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
    }
}

impl fmt::Debug for Rsdp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rsdt_address: u32 = self.rsdt_address;
        let length: u32 = self.length;
        let xsdt_address: u64 = self.xsdt_address;
        formatter
            .debug_struct("Rsdp")
            .field("signature", &self.signature())
            .field("checksum", &self.checksum)
            .field("oemid", &self.oemid())
            .field("revision", &self.revision)
            .field("rsdt_address", &rsdt_address)
            .field("length", &length)
            .field("xsdt_address", &xsdt_address)
            .field("extended_checksum", &self.extended_checksum)
            .field("reserved", &self.reserved)
            .finish()
    }
}

