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
    let rsdp: &acpi::root_system_description::Pointer = Argument::get()
        .efi_system_table()
        .rsdp();
    com2_println!("rsdp = {:#x?}", rsdp);
    let fadt: &acpi::fixed_acpi_description::Table = rsdp
        .xsdt()
        .fadt();
    com2_println!("fadt = {:#x?}", fadt);
    assert_eq!(fadt.pm_tmr_len(), 4);
    0
}

