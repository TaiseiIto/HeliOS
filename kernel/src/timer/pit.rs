//! # Programmable Interval Timer
//! ## References
//! * [Programmable Interval Timer](https://wiki.osdev.org/Programmable_Interval_Timer)
//! * [8254 PROGRAMMABLE INTERVAL TIMER](https://www.scs.stanford.edu/10wi-cs140/pintos/specs/8254.pdf)

use crate::{com2_println, interrupt, x64, Argument};

pub mod control;

const BASE_OSCILATOR_FREQUENCY: usize = 14318180;

pub fn enable_periodic_interrupt(hz: usize) -> u8 {
    let counter: u8 = 0;
    let irq: u8 = 0;
    let divisor: usize = BASE_OSCILATOR_FREQUENCY / (12 * hz);
    assert!(divisor < 0x10000);
    let divisor: u16 = divisor as u16;
    let low: u8 = divisor as u8;
    let high: u8 = (divisor >> u8::BITS) as u8;
    let bcd: bool = false;
    control::Register::create(
        bcd,
        control::Mode::RateGenerator,
        control::Access::LowAndHigh,
        control::Selector::Counter(counter),
    )
    .set();
    x64::port::outb(counter_port(counter), low);
    x64::port::outb(counter_port(counter), high);
    irq
}

pub fn initialize(local_apic_id: u8) {
    let pit_frequency: usize = 0x20; // Hz
    let pit_irq: u8 = enable_periodic_interrupt(pit_frequency);
    com2_println!("pit_irq = {:#x?}", pit_irq);
    Argument::get()
        .efi_system_table_mut()
        .rsdp_mut()
        .xsdt_mut()
        .madt_mut()
        .io_apic_mut()
        .registers_mut()
        .redirect(pit_irq, local_apic_id, interrupt::PIT_INTERRUPT);
}

fn counter_port(index: u8) -> u16 {
    (index as u16) + 0x40
}
