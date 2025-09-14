use bitfield_struct::bitfield;

/// # PCI-X Status Register
/// ## References
/// * [PCI-X Addendum to the PCI Local Bus Specification Revision 1.0](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://catalogue.library.cern/api/files/bd372a28-be4a-44c5-9b5a-6b793fdf2ca0/Fulltext.pdf%3Fdownload&ved=2ahUKEwif4dayjv-MAxU8SfUHHQo6Ky0QFnoECBEQAQ&usg=AOvVaw0pGLYdTmIj33uks6j01ce9) 7.2.4 PCI-X Status Register
#[bitfield(u32)]
pub struct Register {
    #[bits(3, access = RO)]
    function_number: u8,
    #[bits(5, access = RO)]
    device_number: u8,
    #[bits(access = RO)]
    bus_number: u8,
    #[bits(access = RO)]
    bit64_device: bool,
    #[bits(access = RO)]
    mhz133_capable: bool,
    split_completion_discarded: bool,
    unexpected_split_completion: bool,
    #[bits(access = RO)]
    device_complexity: bool,
    #[bits(2, access = RO)]
    designed_maximum_memory_read_byte_count: u8,
    #[bits(3, access = RO)]
    designed_maximum_outstanding_split_transactions: u8,
    #[bits(3, access = RO)]
    designed_maximum_cumulative_read_size: u8,
    received_split_completion_error_message: bool,
    #[bits(2)]
    __: u8,
}
