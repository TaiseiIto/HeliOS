//! # RS232C interrupt enable register
//! ## References
//! * [Look RS232 Interrupt Enable Register](https://www.lookrs232.com/rs232/ier.htm)

use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Register {
    enable_received_data_available_interrupt: bool,
    enable_transmitter_holding_register_empty_interrupt: bool,
    enable_register_line_status_interrupt: bool,
    enable_modem_status_interrupt: bool,
    enable_sleep_mode: bool,
    enable_low_power_mode: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
}

impl Register {
    pub fn disable_all_interrupts(self) -> Self {
        self
            .with_enable_received_data_available_interrupt(false)
            .with_enable_transmitter_holding_register_empty_interrupt(false)
            .with_enable_register_line_status_interrupt(false)
            .with_enable_modem_status_interrupt(false)
            .with_enable_sleep_mode(false)
            .with_enable_low_power_mode(false)
    }
}

