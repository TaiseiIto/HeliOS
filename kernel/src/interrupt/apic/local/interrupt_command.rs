use {
    bitfield_struct::bitfield,
    core::fmt,
};

/// # Interrupt Command Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.1 Figure 11-12. Interrupt Command Register (ICR)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    low: FatLow,
    high: FatHigh,
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct FatLow {
    register: Low,
    reserved0: [u32; 3],
}

impl fmt::Debug for FatLow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Low = self.register;
        let vector: u8 = register.vector();
        let delivery_mode: u8 = register.delivery_mode();
        let destination_mode: bool = register.destination_mode();
        let delivery_status: bool = register.delivery_status();
        let level: bool = register.level();
        let trigger_mode: bool = register.trigger_mode();
        let destination_shorthand: u8 = register.destination_shorthand();
        formatter
            .debug_struct("Low")
            .field("vector", &vector)
            .field("delivery_mode", &delivery_mode)
            .field("destination_mode", &destination_mode)
            .field("delivery_status", &delivery_status)
            .field("level", &level)
            .field("trigger_mode", &trigger_mode)
            .field("destination_shorthand", &destination_shorthand)
            .finish()
    }
}

#[bitfield(u32)]
struct Low {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    destination_mode: bool,
    delivery_status: bool,
    #[bits(access = RO)]
    reserved0: bool,
    level: bool,
    trigger_mode: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
    #[bits(2)]
    destination_shorthand: u8,
    #[bits(12, access = RO)]
    reserved2: u16,
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct FatHigh {
    register: High,
    reserved0: [u32; 3],
}

impl fmt::Debug for FatHigh {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: High = self.register;
        let destination_field: u8 = register.destination_field();
        formatter
            .debug_struct("High")
            .field("destination_field", &destination_field)
            .finish()
    }
}

#[bitfield(u32)]
struct High {
    #[bits(24, access = RO)]
    reserved0: u32,
    destination_field: u8,
}

