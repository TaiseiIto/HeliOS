/// # EFI_TIME
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
#[repr(C)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    day_light: u8,
    pad2: u8,
}

/// # EFI_TIME_CAPABILITIES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
#[repr(C)]
pub struct Capabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool,
}

/// # EFI_TIMER_DELAY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
#[derive(Debug)]
#[repr(C)]
pub enum Delay {
    Cancel,
    Periodic,
    Relative,
}

