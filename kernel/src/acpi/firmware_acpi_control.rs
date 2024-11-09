use {
    bitfield_struct::bitfield,
    core::{
        fmt,
        str,
    },
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
    global_lock: GlobalLock,
    flags: Flags,
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
        let global_lock: GlobalLock = self.global_lock;
        let flags: Flags = self.flags;
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

/// # Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.10 Table 5.14 Firmware Control Structure Feature Flags
#[bitfield(u32)]
struct Flags {
    s4bios_f: bool,
    bit64_wake_supported_f: bool,
    #[bits(30)]
    __: u32,
}

/// # Global Lock
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.10.1 Global Lock
#[bitfield(u32)]
struct GlobalLock {
    pending: bool,
    owned: bool,
    #[bits(30)]
    __: u32,
}

