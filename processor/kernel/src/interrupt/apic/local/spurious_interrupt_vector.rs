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
    pub fn enable(&mut self, focus_processor_checking: bool, eoi_broadcast: bool, spurious_vector: u8) {
        let apic_software_enable: bool = true;
        *self.register_mut() = Register::create(apic_software_enable, focus_processor_checking, eoi_broadcast, spurious_vector).into();
    }

    fn register_mut(&mut self) -> &mut u32 {
        let register: *mut Self = self as *mut Self;
        let register: *mut u32 = register as *mut u32;
        unsafe {
            &mut *register
        }
    }
}

impl fmt::Debug for FatRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let register: Register = self.register;
        let spurious_vector: u8 = register.spurious_vector();
        let apic_software_enable: bool = register.apic_software_enable();
        let focus_processor_checking_disable: bool = register.focus_processor_checking_disable();
        let eoi_broadcast_suppression: bool = register.eoi_broadcast_suppression();
        formatter
            .debug_struct("Register")
            .field("spurious_vector", &spurious_vector)
            .field("apic_software_enable", &apic_software_enable)
            .field("focus_processor_checking_disable", &focus_processor_checking_disable)
            .field("eoi_broadcast_suppression", &eoi_broadcast_suppression)
            .finish()
    }
}

/// # Spurious Interrupt Vector Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.9 Figure 11-23. Spurious Interrupt Vector Register (SVR)
#[bitfield(u32)]
struct Register {
    spurious_vector: u8,
    apic_software_enable: bool,
    focus_processor_checking_disable: bool,
    #[bits(2)]
    __: u8,
    eoi_broadcast_suppression: bool,
    #[bits(19)]
    __: u32,
}

impl Register {
    fn create(apic_software_enable: bool, focus_processor_checking: bool, eoi_broadcast: bool, spurious_vector: u8) -> Self {
        let focus_processor_checking_disable: bool = !focus_processor_checking;
        let eoi_broadcast_suppression: bool = !eoi_broadcast;
        Self::new()
            .with_spurious_vector(spurious_vector)
            .with_apic_software_enable(apic_software_enable)
            .with_focus_processor_checking_disable(focus_processor_checking_disable)
            .with_eoi_broadcast_suppression(eoi_broadcast_suppression)
    }
}

