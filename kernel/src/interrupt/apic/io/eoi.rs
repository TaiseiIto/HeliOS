use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    reserved0: [u32; 3],
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let redirection_entry_clear: u32 = register.redirection_entry_clear();
        formatter
            .debug_struct("Register")
            .field("redirection_entry_clear", &redirection_entry_clear)
            .finish()
    }
}

/// # EOI Register (EOIR) - Offset FEC00040h
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2.3 EOI Register (EOIR) - Offset FEC00040h
#[bitfield(u32)]
struct Register {
    redirection_entry_clear: u32,
}

