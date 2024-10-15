use bitfield_struct::bitfield;

/// # PM1 Control Register
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 4.8.3.2.1 PM1 Control Registers
#[bitfield(u16)]
pub struct Register {
    sci_en: bool,
    bm_rld: bool,
    gbl_rls: bool,
    #[bits(6, access = RO)]
    reserved0: u8,
    ignore: bool,
    #[bits(3)]
    slp_typx: u8,
    slp_en: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
}

