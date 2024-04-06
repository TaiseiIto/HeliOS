pub mod data;
pub mod eoi;
pub mod index;

use core::fmt;

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
    reserved0: [u128; 2],
    // 0xfec00040
    eoi: eoi::FatRegister,
}

impl Registers {
    pub fn get_u32(&mut self, index: u8) -> u32 {
        self.index.set(index);
        self.data.get()
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

