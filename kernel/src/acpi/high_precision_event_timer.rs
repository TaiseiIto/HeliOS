use {
    bitfield_struct::bitfield,
    core::fmt,
    crate::timer,
    super::{
        generic_address,
        system_description,
    },
};

/// # HPET
/// ## References
/// * [IA-PC HPET (High Precision Event Timers) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 3.2.4 The ACPI 2.0 HPET Description Table (HPET)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    event_timer_block_id: EventTimerBlockId,
    base_address: generic_address::Structure,
    hpet_number: u8,
    main_counter_minimum_clock_tick_in_periodic_mode: u16,
    page_protection_and_oem_attribute: u8,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn registers(&self) -> &timer::hpet::Registers {
        self.base_address.address()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let event_timer_block_id: EventTimerBlockId = self.event_timer_block_id;
        let registers: &timer::hpet::Registers = self.registers();
        let hpet_number: u8 = self.hpet_number;
        let main_counter_minimum_clock_tick_in_periodic_mode: u16 = self.main_counter_minimum_clock_tick_in_periodic_mode;
        let page_protection_and_oem_attribute: u8 = self.page_protection_and_oem_attribute;
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("event_timer_block_id", &event_timer_block_id)
            .field("registers", registers)
            .field("hpet_number", &hpet_number)
            .field("main_counter_minimum_clock_tick_in_periodic_mode", &main_counter_minimum_clock_tick_in_periodic_mode)
            .field("page_protection_and_oem_attribute", &page_protection_and_oem_attribute)
            .finish()
    }
}

/// # Event Timer Block ID
/// ## References
/// * [IA-PC HPET (High Precision Event Timers) Specification](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/software-developers-hpet-spec-1-0a.pdf) 3.2.4 Table 3 HPET Description Table
#[bitfield(u32)]
struct EventTimerBlockId {
    hardware_rev_id: u8,
    #[bits(5)]
    number_of_comparators_in_first_timer_block: u8,
    counter_size_cap_counter_size: bool,
    reserved0: bool,
    legacy_placement_irq_routing_capable: bool,
    pci_vendor_id_of_first_timer_block: u16,
}

