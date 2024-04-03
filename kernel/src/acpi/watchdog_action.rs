mod instruction;

use {
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # Watchdog Action Table (WDAT)
/// ## References
/// * [Hardware Watchdog Timers Design Specification](https://download.microsoft.com/download/a/f/7/af7777e5-7dcd-4800-8a0a-b18336565f5b/hardwarewdtspec.doc)
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
    flags: Flags,
    reserved1: [u8; 3],
    number_watchdog_instruction_entries: u32,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn instruction_entries(&self) -> &[instruction::Entry] {
        let table: *const Self = self as *const Self;
        let instruction_entries: *const Self = unsafe {
            table.add(1)
        };
        let instruction_entries: *const instruction::Entry = instruction_entries as *const instruction::Entry;
        let length: usize = (self.header.table_size() - mem::size_of::<Self>()) / mem::size_of::<instruction::Entry>();
        unsafe {
            slice::from_raw_parts(instruction_entries, length)
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let watchdog_header_length: u32 = self.watchdog_header_length;
        let pci_segment: u16 = self.pci_segment;
        let pci_bus_number: u8 = self.pci_bus_number;
        let pci_device_number: u8 = self.pci_device_number;
        let pci_function_number: u8 = self.pci_function_number;
        let reserved0: [u8; 3] = self.reserved0;
        let timer_period: u32 = self.timer_period;
        let maximum_count: u32 = self.maximum_count;
        let minimum_count: u32 = self.minimum_count;
        let flags: Flags = self.flags;
        let reserved1: [u8; 3] = self.reserved1;
        let number_watchdog_instruction_entries: u32 = self.number_watchdog_instruction_entries;
        let instruction_entries: &[instruction::Entry] = self.instruction_entries();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("watchdog_header_length", &watchdog_header_length)
            .field("pci_segment", &pci_segment)
            .field("pci_bus_number", &pci_bus_number)
            .field("pci_device_number", &pci_device_number)
            .field("pci_function_number", &pci_function_number)
            .field("reserved0", &reserved0)
            .field("timer_period", &timer_period)
            .field("maximum_count", &maximum_count)
            .field("minimum_count", &minimum_count)
            .field("flags", &flags)
            .field("reserved1", &reserved1)
            .field("number_watchdog_instruction_entries", &number_watchdog_instruction_entries)
            .field("instruction_entries", &instruction_entries)
            .finish()
    }
}

#[bitfield(u8)]
struct Flags {
    enabled: bool,
    #[bits(6, access = RO)]
    reserved0: u8,
    stopped_in_sleep_state: bool,
}

