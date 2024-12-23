pub mod data;
pub mod eoi;
pub mod identification;
pub mod index;
pub mod redirection;
pub mod version;

use {
    alloc::vec::Vec,
    core::fmt,
};

/// # Advanced Programmable Interrupt Controller (APIC) Registers
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2 Advanced Programmable Interrupt Controller (APIC) Registers Summary
#[repr(packed)]
pub struct Registers {
    // 0xfec00000
    index: index::FatRegister,
    // 0xfec00010
    data: data::FatRegister,
    // 0xfec00020
    __: [u128; 2],
    // 0xfec00040
    eoi: eoi::FatRegister,
}

impl Registers {
    pub fn end_interruption(&mut self, interrupt_number: u8) {
        self.eoi.ends_interruption(interrupt_number);
    }

    pub fn identification(&mut self) -> identification::Register {
        self.get_u32(0).into()
    }

    pub fn redirection_table_entries(&mut self) -> Vec<redirection::table::Entry> {
        (0..)
            .map_while(|irq| self.redirection_table_entry(irq))
            .collect()
    }

    pub fn redirect(&mut self, irq: u8, local_apic_id: u8, interrupt_number: u8) {
        let redirection_table_entry: redirection::table::Entry = self
            .redirection_table_entry(irq)
            .unwrap()
            .with_redirection(local_apic_id, interrupt_number);
        self.set_redirection_table_entry(irq, redirection_table_entry);
    }

    pub fn redirection_table_entry(&mut self, irq: u8) -> Option<redirection::table::Entry> {
        ((irq as usize) < self.version().redirection_table_length()).then(|| {
            let index: u8 = 0x10 + 2 * irq;
            let low_index: u8 = index;
            let high_index: u8 = index + 1;
            let low: u32 = self.get_u32(low_index);
            let high: u32 = self.get_u32(high_index);
            let redirection_table_entry: u64 = (low as u64) + ((high as u64) << u32::BITS);
            redirection_table_entry.into()
        })
    }

    pub fn version(&mut self) -> version::Register {
        self.get_u32(1).into()
    }

    fn get_u32(&mut self, index: u8) -> u32 {
        self.index.set(index);
        self.data.get()
    }

    fn set_redirection_table_entry(&mut self, irq: u8, redirection_table_entry: redirection::table::Entry) {
        assert!((irq as usize) < self.version().redirection_table_length());
        let redirection_table_entry: u64 = redirection_table_entry.into();
        let index: u8 = 0x10 + 2 * irq;
        let low_index: u8 = index;
        let high_index: u8 = index + 1;
        let low_redirection_table_entry: u32 = (redirection_table_entry & 0x00000000ffffffff) as u32;
        let high_redirection_table_entry: u32 = (redirection_table_entry >> u32::BITS) as u32;
        self.set_u32(low_index, low_redirection_table_entry);
        self.set_u32(high_index, high_redirection_table_entry);
    }

    fn set_u32(&mut self, index: u8, data: u32) {
        self.index.set(index);
        self.data.set(data);
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index: index::FatRegister = self.index;
        let data: data::FatRegister = self.data;
        let eoi: eoi::FatRegister = self.eoi;
        formatter
            .debug_struct("Registers")
            .field("index", &index)
            .field("data", &data)
            .field("eoi", &eoi)
            .finish()
    }
}

