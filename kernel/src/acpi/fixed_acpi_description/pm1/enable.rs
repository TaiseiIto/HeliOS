use bitfield_struct::bitfield;

/// # PM1 Enable Register
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 4.8.3.1.2 PM1 Enable Registers
#[bitfield(u16)]
pub struct Register {
    tmr_en: bool,
    #[bits(4, access = RO)]
    reserved0: u8,
    gbl_en: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
    pwrbtn_en: bool,
    slpbtn_en: bool,
    rtc_en: bool,
    #[bits(3, access = RO)]
    reserved2: u8,
    pciexp_wake_dis: bool,
    #[bits(access = RO)]
    reserved3: bool,
}

