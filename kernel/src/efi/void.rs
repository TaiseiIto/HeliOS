use super::memory;

pub const VOID: Void = Void;

pub fn null<T>() -> &'static T {
    let null: usize = 0;
    let null: *const T = null as *const T;
    unsafe {
        &*null
    }
}

/// # CHAR16
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
#[derive(Debug)]
pub struct Void;

impl Void {
    pub fn is_null(&self) -> bool {
        let void: *const Self = self as *const Self;
        let void: usize = void as usize;
        void == 0
    }
}

impl From<memory::PhysicalAddress> for &Void {
    fn from(pointer: u64) -> Self {
        let pointer: *const Void = pointer as *const Void;
        unsafe {
            &*pointer
        }
    }
}

impl From<*mut u8> for &Void {
    fn from(pointer: *mut u8) -> Self {
        let pointer: usize = pointer as usize;
        let pointer: *const Void = pointer as *const Void;
        unsafe {
            &*pointer
        }
    }
}

impl From<&Void> for *mut u8 {
    fn from(pointer: &Void) -> Self {
        let pointer: *const Void = pointer as *const Void;
        let pointer: usize = pointer as usize;
        pointer as Self
    }
}

