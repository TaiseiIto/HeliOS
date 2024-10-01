//! # Non Maskable Interrupt
//! ## References
//! * [Non Maskable Interrupt](https://wiki.osdev.org/Non_Maskable_Interrupt)

use {
    core::cell::UnsafeCell,
    crate::{
        task,
        x64,
    },
};

pub const DISABLE: u8 = 0x80;

static mut ENABLED: UnsafeCell<bool> = UnsafeCell::new(true);

pub fn disable() {
    task::Controller::get_current_mut()
        .unwrap()
        .cli();
    unsafe {
        *ENABLED.get_mut() = false;
    }
    let address: u8 = x64::port::inb(x64::cmos::ADDRESS_PORT);
    let address: u8 = address | DISABLE;
    x64::port::outb(x64::cmos::ADDRESS_PORT, address);
    x64::port::inb(x64::cmos::DATA_PORT);
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
}

pub fn enable() {
    task::Controller::get_current_mut()
        .unwrap()
        .cli();
    unsafe {
        *ENABLED.get_mut() = true;
    }
    let address: u8 = x64::port::inb(x64::cmos::ADDRESS_PORT);
    let address: u8 = address & !DISABLE;
    x64::port::outb(x64::cmos::ADDRESS_PORT, address);
    x64::port::inb(x64::cmos::DATA_PORT);
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
}

pub fn is_enabled() -> bool {
    unsafe {
        *ENABLED.get()
    }
}

