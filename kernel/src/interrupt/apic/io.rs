pub mod data;
pub mod eoi;
pub mod index;

/// # Advanced Programmable Interrupt Controller (APIC) Registers
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2 Advanced Programmable Interrupt Controller (APIC) Registers Summary
#[derive(Debug)]
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

