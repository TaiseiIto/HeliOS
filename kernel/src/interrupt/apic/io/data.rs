use {bitfield_struct::bitfield, core::fmt};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    __: [u32; 3],
}

impl FatRegister {
    pub fn get(&self) -> u32 {
        let register: Register = self.register;
        register.data()
    }

    pub fn set(&mut self, data: u32) {
        let register: *mut Self = self as *mut Self;
        let register: *mut u32 = register as *mut u32;
        unsafe {
            register.write_volatile(data);
        }
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let data: u32 = register.data();
        formatter
            .debug_struct("Register")
            .field("data", &data)
            .finish()
    }
}

/// # Data Register (DAT) - Offset FEC00010h
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.2.2 Data Register (DAT) - Offset FEC00010h
#[bitfield(u32)]
struct Register {
    data: u32,
}
