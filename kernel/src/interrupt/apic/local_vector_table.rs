//! # Local Vector Table (LVT)
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.1 Figure 11-8. Local Vector Table (LVT)

use bitfield_struct::bitfield;

#[bitfield(u128)]
pub struct Register {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    #[bits(access = RO)]
    reserved0: bool,
    delivery_status: bool,
    interrupt_input_pin_polarity: bool,
    remote_irr: bool,
    trigger_mode: bool,
    mask: bool,
    #[bits(111, access = RO)]
    reserved1: u128,
}

