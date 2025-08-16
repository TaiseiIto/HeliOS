use bitfield_struct::bitfield;

/// # PCI Express Device Status 2 Register
/// ## References
/// * [ntddk.h](https://codemachine.com/downloads/win10.1511/ntddk.h)
#[bitfield(u16)]
pub struct Register {
    __: u16,
}

