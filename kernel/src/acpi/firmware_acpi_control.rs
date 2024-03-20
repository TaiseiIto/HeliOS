use core::{
    fmt,
    str,
};

/// # FACS
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.10 Firmware ACPI Control Structure (FACS)
#[repr(packed)]
pub struct Structure {
    signature: [u8; 4],
    length: u32,
    hardware_signature: u32,
    firmware_waking_vendor: u32,
    global_lock: u32,
    flags: u32,
    x_firmware_waking_vendor: u64,
}

impl Structure {
    fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length: u32 = self.length;
        let hardware_signature: u32 = self.hardware_signature;
        let firmware_waking_vendor: u32 = self.firmware_waking_vendor;
        let global_lock: u32 = self.global_lock;
        let flags: u32 = self.flags;
        let x_firmware_waking_vendor: u64 = self.x_firmware_waking_vendor;
        formatter
            .debug_struct("Structure")
            .field("signature", &self.signature())
            .field("length", &length)
            .field("hardware_signature", &hardware_signature)
            .field("firmware_waking_vendor", &firmware_waking_vendor)
            .field("global_lock", &global_lock)
            .field("flags", &flags)
            .field("x_firmware_waking_vendor", &x_firmware_waking_vendor)
            .finish()
    }
}

