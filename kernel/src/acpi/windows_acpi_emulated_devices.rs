use {super::system_description, bitfield_struct::bitfield};

/// # WAET
/// ## References
/// * [Windows ACPI Emulated Devices Table](https://learn.microsoft.com/en-us/previous-versions/gg487524(v=msdn.10))
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    #[allow(dead_code)]
    emulated_device_flags: Flags,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

/// # Emulated Device Flags
/// ## References
/// * [Windows ACPI Emulated Devices Table](https://learn.microsoft.com/en-us/previous-versions/gg487524(v=msdn.10))
#[bitfield(u32)]
struct Flags {
    rtc_good: bool,
    acpi_tm_timer_good: bool,
    #[bits(30)]
    __: u32,
}
