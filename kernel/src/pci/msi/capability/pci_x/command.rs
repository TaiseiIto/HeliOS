use bitfield_struct::bitfield;

/// # PCI-X Command Register
/// ## References
/// * [PCI-X Addendum to the PCI Local Bus Specification Revision 1.0](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://catalogue.library.cern/api/files/bd372a28-be4a-44c5-9b5a-6b793fdf2ca0/Fulltext.pdf%3Fdownload&ved=2ahUKEwif4dayjv-MAxU8SfUHHQo6Ky0QFnoECBEQAQ&usg=AOvVaw0pGLYdTmIj33uks6j01ce9) 7.2.3 PCI-X Command Register
#[bitfield(u16)]
pub struct Register {
    data_parity_error_recovery_enable: bool,
    enable_relaxed_ordering: bool,
    #[bits(2)]
    maximum_memory_read_byte_count: u8,
    #[bits(3)]
    maximum_outstanding_split_transactions: u8,
    #[bits(9)]
    __: u16,
}

