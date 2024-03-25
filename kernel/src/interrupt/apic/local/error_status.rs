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

