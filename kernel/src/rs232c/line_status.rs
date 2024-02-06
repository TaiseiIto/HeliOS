//! # RS232C line status register
//! ## References
//! * [Look RS232 Line Status Register](https://www.lookrs232.com/rs232/lsr.htm)

use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Register {
    data_ready: bool,
    overrun_error: bool,
    parity_error: bool,
    framing_error: bool,
    break_interrupt: bool,
    empty_transmitter_holding: bool,
    empty_data_holding: bool,
    error_in_received_fifo: bool,
}

impl Register {
    pub fn can_send(self) -> bool {
        self.empty_transmitter_holding()
    }
}

