//! # Time Stamp Counter
//! ## References
//! * [TSC](https://wiki.osdev.org/TSC)

use crate::x64;

pub fn counter_value() -> u64 {
    x64::rdtsc()
}

