//! # ACPI Power Management Timer
//! ## References
//! * [ACPI Timer](https://wiki.osdev.org/ACPI_Timer)

use crate::{
    Argument,
    acpi,
    com2_print,
    com2_println,
};

pub fn read_counter_value() -> u32 {
    Argument::get()
        .efi_system_table()
        .rsdp()
        .xsdt()
        .fadt()
        .acpi_timer()
        .unwrap()
        .read_u32()
}

