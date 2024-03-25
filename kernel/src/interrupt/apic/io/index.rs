use bitfield_struct::bitfield;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

/// # Index Register (IDX) - Offset FEC00000h
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2.1 Index Register (IDX) - Offset FEC00000h
#[bitfield(u32)]
struct Register {
    index: u8,
    #[bits(24, access = RO)]
    reserved: u32,
}

