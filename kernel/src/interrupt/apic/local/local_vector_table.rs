use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

/// # Local Vector Table (LVT)
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.1 Figure 11-8. Local Vector Table (LVT)
#[bitfield(u32)]
struct Register {
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
    #[bits(2)]
    timer_mode: u8,
    #[bits(13, access = RO)]
    reserved1: u16,
}

