use {
    bitfield_struct::bitfield,
    super::system_description,
};

/// # Watchdog Action Table (WDAT)
/// ## References
/// * [Hardware Watchdog Timers Design Specification](https://download.microsoft.com/download/a/f/7/af7777e5-7dcd-4800-8a0a-b18336565f5b/hardwarewdtspec.doc)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    watchdog_header_length: u32,
    pci_segment: u16,
    pci_bus_number: u8,
    pci_device_number: u8,
    pci_function_number: u8,
    reserved0: [u8; 3],
    timer_period: u32,
    maximum_count: u32,
    minimum_count: u32,
    watchdog_flags: Flags,
    reserved1: [u8; 3],
    number_watchdog_instruction_entries: u32,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

#[bitfield(u8)]
struct Flags {
    enabled: bool,
    #[bits(6, access = RO)]
    reserved0: u8,
    stopped_in_sleep_state: bool,
}

