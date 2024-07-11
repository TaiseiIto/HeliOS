//! # ACPI Power Management Timer
//! ## References
//! * [ACPI Timer](https://wiki.osdev.org/ACPI_Timer)

use crate::{
    Argument,
    x64,
};

#[allow(dead_code)]
const FREQUENCY: usize = 3579545; // Hz

pub fn counter_value() -> u32 {
    Argument::get()
        .efi_system_table()
        .rsdp()
        .xsdt()
        .fadt()
        .timer()
        .unwrap()
        .read_u32()
}

pub fn bits() -> usize {
    Argument::get()
        .efi_system_table()
        .rsdp()
        .xsdt()
        .fadt()
        .timer_bits()
}

#[allow(dead_code)]
pub fn wait_microseconds(microseconds: usize) {
    let current_counter_value: usize = counter_value() as usize;
    let bits: usize = bits();
    let increments: usize = microseconds * FREQUENCY / 1000000;
    assert!(increments < 1 << (bits - 1));
    let minimum_counter_value: usize = (current_counter_value + increments) % (1 << bits);
    let maximum_counter_value: usize = (minimum_counter_value + (1 << (bits - 1))) % (1 << bits);
    while {
        let current_counter_value: usize = counter_value() as usize;
        if minimum_counter_value < maximum_counter_value {
            !(minimum_counter_value..maximum_counter_value).contains(&current_counter_value)
        } else {
            (maximum_counter_value..minimum_counter_value).contains(&current_counter_value)
        }
    } {
        x64::pause();
    }
}

#[allow(dead_code)]
pub fn wait_milliseconds(milliseconds: usize) {
    wait_microseconds(1000 * milliseconds)
}

#[allow(dead_code)]
pub fn wait_seconds(seconds: usize) {
    wait_milliseconds(1000 * seconds)
}

