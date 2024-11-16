use {
    bitfield_struct::bitfield,
    crate::interrupt,
    super::super::{
        rdmsr,
        super::Cpuid,
        wrmsr,
    },
};

/// # IA32_APIC_BASE
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-4
#[bitfield(u64)]
pub struct ApicBase {
    __: u8,
    bsp: bool,
    __: bool,
    enable_x2apic_mode: bool,
    apic_global_enable: bool,
    #[bits(52)]
    apic_base: u64,
}

impl ApicBase {
    const ECX: u32 = 0x0000001b;

    pub fn enable(&mut self) {
        self.set_apic_global_enable(true);
        wrmsr(Self::ECX, (*self).into());
    }

    pub fn get(cpuid: &Cpuid) -> Option<Self> {
        cpuid
            .supports_apic()
            .then(|| rdmsr(Self::ECX)
                .into())
    }

    pub fn registers(&self) -> &interrupt::apic::local::Registers {
        let registers: usize = (self.apic_base() as usize) << Self::APIC_BASE_OFFSET;
        let registers: *const interrupt::apic::local::Registers = registers as *const interrupt::apic::local::Registers;
        unsafe {
            &*registers
        }
    }

    pub fn registers_mut(&mut self) -> &mut interrupt::apic::local::Registers {
        let registers: usize = (self.apic_base() as usize) << Self::APIC_BASE_OFFSET;
        let registers: *mut interrupt::apic::local::Registers = registers as *mut interrupt::apic::local::Registers;
        unsafe {
            &mut *registers
        }
    }
}

