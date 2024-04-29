use {
    bitfield_struct::bitfield,
    core::fmt,
};

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct FatRegister {
    register: Register,
    #[allow(dead_code)]
    reserved0: [u32; 3],
}

impl FatRegister {
    pub fn clear_all_errors(&mut self) {
        let register: Register = self.register;
        let register: Register = register.clear_all_errors();
        *self.register_mut() = register.into();
    }

    fn address(&self) -> usize {
        let address: *const Self = self as *const Self;
        address as usize
    }

    fn register_mut(&mut self) -> &mut u32 {
        let register: usize = self.address();
        let register: *mut u32 = register as *mut u32;
        unsafe {
            &mut *register
        }
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let send_checksum_error: bool = register.send_checksum_error();
        let receive_checksum_error: bool = register.receive_checksum_error();
        let send_accept_error: bool = register.send_accept_error();
        let receive_accept_error: bool = register.receive_accept_error();
        let redirectable_ipi: bool = register.redirectable_ipi();
        let send_illegal_vector: bool = register.send_illegal_vector();
        let received_illegal_vector: bool = register.received_illegal_vector();
        let illegal_register_address: bool = register.illegal_register_address();
        formatter
            .debug_struct("Register")
            .field("register", &register)
            .field("send_checksum_error", &send_checksum_error)
            .field("receive_checksum_error", &receive_checksum_error)
            .field("send_accept_error", &send_accept_error)
            .field("receive_accept_error", &receive_accept_error)
            .field("redirectable_ipi", &redirectable_ipi)
            .field("send_illegal_vector", &send_illegal_vector)
            .field("received_illegal_vector", &received_illegal_vector)
            .field("illegal_register_address", &illegal_register_address)
            .finish()
    }
}

/// # Error Status Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.3 Figure 11-9. Error Status Register (ESR)
#[bitfield(u32)]
struct Register {
    send_checksum_error: bool,
    receive_checksum_error: bool,
    send_accept_error: bool,
    receive_accept_error: bool,
    redirectable_ipi: bool,
    send_illegal_vector: bool,
    received_illegal_vector: bool,
    illegal_register_address: bool,
    #[bits(24, access = RO)]
    reserved0: u32,
}

impl Register {
    pub fn clear_all_errors(self) -> Self {
        self.with_send_checksum_error(false)
            .with_receive_checksum_error(false)
            .with_send_accept_error(false)
            .with_receive_accept_error(false)
            .with_redirectable_ipi(false)
            .with_send_illegal_vector(false)
            .with_received_illegal_vector(false)
            .with_illegal_register_address(false)
    }
}

