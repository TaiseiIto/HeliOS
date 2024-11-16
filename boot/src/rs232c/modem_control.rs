//! # RS232C modem control register
//! ## References
//! * [Look RS232 Model Control Register](https://www.lookrs232.com/rs232/mcr.htm)

use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct Register {
    force_data_terminal_ready: bool,
    force_request_to_send: bool,
    aux_output1: bool,
    aux_output2: bool,
    loopback_mode: bool,
    autoflow_control_enabled: bool,
    #[bits(2)]
    __: u8,
}

impl Register {
    pub fn create(
        force_data_terminal_ready: bool,
        force_request_to_send: bool,
        aux_output1: bool,
        aux_output2: bool,
        loopback_mode: bool,
        autoflow_control_enabled: bool,
    ) -> Self {
        Self::new()
            .with_force_data_terminal_ready(force_data_terminal_ready)
            .with_force_request_to_send(force_request_to_send)
            .with_aux_output1(aux_output1)
            .with_aux_output2(aux_output2)
            .with_loopback_mode(loopback_mode)
            .with_autoflow_control_enabled(autoflow_control_enabled)
    }
}

