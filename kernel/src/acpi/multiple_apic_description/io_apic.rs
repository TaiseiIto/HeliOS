use {
    core::fmt,
    crate::interrupt,
};

/// # I/O APIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.3 I/O APIC Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    io_apic_id: u8,
    reserved0: u8,
    io_apic_address: u32,
    global_system_interrupt_base: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn registers(&self) -> &interrupt::apic::io::Registers {
        let registers: u32 = self.io_apic_address;
        let registers: usize = registers as usize;
        let registers: *const interrupt::apic::io::Registers = registers as *const interrupt::apic::io::Registers;
        unsafe {
            &*registers
        }
    }

    pub fn registers_mut(&mut self) -> &mut interrupt::apic::io::Registers {
        let registers: u32 = self.io_apic_address;
        let registers: usize = registers as usize;
        let registers: *mut interrupt::apic::io::Registers = registers as *mut interrupt::apic::io::Registers;
        unsafe {
            &mut *registers
        }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u8 = self.structure_type;
        let length: u8 = self.length;
        let io_apic_id: u8 = self.io_apic_id;
        let reserved0: u8 = self.reserved0;
        let registers: &interrupt::apic::io::Registers = self.registers();
        let global_system_interrupt_base: u32 = self.global_system_interrupt_base;
        formatter
            .debug_struct("Struct")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("io_apic_id", &io_apic_id)
            .field("reserved0", &reserved0)
            .field("registers", registers)
            .field("global_system_interrupt_base", &global_system_interrupt_base)
            .finish()
    }
}

