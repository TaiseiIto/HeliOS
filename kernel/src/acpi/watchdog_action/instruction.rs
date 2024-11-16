use super::super::generic_address;

/// # Watchdog Instruction Entry
/// ## References
/// * [Hardware Watchdog Timers Design Specification](https://download.microsoft.com/download/a/f/7/af7777e5-7dcd-4800-8a0a-b18336565f5b/hardwarewdtspec.doc)
#[derive(Debug)]
#[repr(packed)]
pub struct Entry {
    #[allow(dead_code)]
    action: u8,
    #[allow(dead_code)]
    flags: u8,
    __: u16,
    #[allow(dead_code)]
    register_region: generic_address::Structure,
    #[allow(dead_code)]
    value: u32,
    #[allow(dead_code)]
    mask: u32,
}

