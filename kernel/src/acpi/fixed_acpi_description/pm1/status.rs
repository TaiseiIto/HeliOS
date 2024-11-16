use bitfield_struct::bitfield;

/// # PM1 Status Register
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 4.8.3.1.1 PM1 Status Registers
#[bitfield(u16)]
pub struct Register {
    tmr_sts: bool,
    #[bits(3)]
    __: u8,
    bm_sts: bool,
    gbl_sts: bool,
    #[bits(2)]
    __: u8,
    pwrbtn_sts: bool,
    slpbtn_sts: bool,
    rtc_sts: bool,
    ignore: bool,
    #[bits(2)]
    __: u8,
    pciexp_wake_sts: bool,
    wak_sts: bool,
}

