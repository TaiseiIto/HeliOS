//! RS232C FIFO control register

use bitfield_struct::bitfield;

// https://www.lookrs232.com/rs232/fcr.htm
#[bitfield(u8)]
pub struct Register {
    enable_fifo: bool,
    clear_receive_fifo: bool,
    clear_transmit_fifo: bool,
    dma_mode_select: bool,
    #[bits(default = false)]
    reserved: bool,
    enable_64byte_fifo: bool,
    #[bits(2)]
    interrupt_trigger_level: u8,
}

impl Register {
    pub fn create(
        enable_fifo: bool,
        clear_receive_fifo: bool,
        clear_transmit_fifo: bool,
        dma_mode_select: bool,
        enable_64byte_fifo: bool,
        interrupt_trigger_level: u8,
    ) -> Self {
        Self::new()
            .with_enable_fifo(enable_fifo)
            .with_clear_receive_fifo(clear_receive_fifo)
            .with_clear_transmit_fifo(clear_transmit_fifo)
            .with_dma_mode_select(dma_mode_select)
            .with_enable_64byte_fifo(enable_64byte_fifo)
            .with_interrupt_trigger_level(match interrupt_trigger_level {
                1 => 0b00,
                4 => 0b01,
                8 => 0b10,
                14 => 0b11,
                _ => panic!("Invalid interrupt trigger level!"),
            })
    }
}

