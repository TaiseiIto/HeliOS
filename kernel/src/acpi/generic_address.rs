/// # Generic Address Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.3.2 Generic Address Structure
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Structure {
    address_space_id: u8,
    register_bit_width: u8,
    register_bit_offset: u8,
    access_size: u8,
    address: u64,
}

impl Structure {
    pub fn address<T>(&self) -> &T {
        let address: usize = self.address as usize;
        let address: *const T = address as *const T;
        unsafe {
            &*address
        }
    }
}

