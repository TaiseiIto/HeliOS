use super::super::generic_address;

/// # Watchdog Instruction Entry
/// ## References
/// * [Hardware Watchdog Timers Design Specification](https://download.microsoft.com/download/a/f/7/af7777e5-7dcd-4800-8a0a-b18336565f5b/hardwarewdtspec.doc)
#[derive(Debug)]
#[repr(packed)]
pub struct Entry {
    action: u8,
    flags: u8,
    reserved0: u16,
    register_region: generic_address::Structure,
    value: u32,
    mask: u32,
}

