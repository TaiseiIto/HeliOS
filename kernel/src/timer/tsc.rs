//! # Time Stamp Counter
//! ## References
//! * [TSC](https://wiki.osdev.org/TSC)

use crate::{
    Argument,
    x64,
};

pub fn counter_value() -> u64 {
    x64::rdtsc()
}

pub fn is_invariant() -> bool {
    Argument::get()
        .cpuid()
        .tsc_is_invariant()
}

