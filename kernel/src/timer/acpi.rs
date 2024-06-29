//! # ACPI Power Management Timer
//! ## References
//! * [ACPI Timer](https://wiki.osdev.org/ACPI_Timer)

use crate::{
    Argument,
    acpi,
};

pub fn read_counter_value() -> u32 {
    let fadt: &acpi::fixed_acpi_description::Table = Argument::get()
        .efi_system_table()
        .rsdp()
        .xsdt()
        .fadt();
    assert_eq!(fadt.pm_tmr_len(), 4);
    0
}

