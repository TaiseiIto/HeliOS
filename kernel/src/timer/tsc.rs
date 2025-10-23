//! # Time Stamp Counter
//! ## References
//! * [TSC](https://wiki.osdev.org/TSC)

use crate::{x64, Argument};

#[allow(dead_code)]
pub fn counter_value() -> u64 {
    x64::rdtsc()
}

#[allow(dead_code)]
pub fn frequency() -> Option<u64> {
    Argument::get().cpuid().tsc_frequency()
}

#[allow(dead_code)]
pub fn is_invariant() -> bool {
    Argument::get().cpuid().tsc_is_invariant()
}
