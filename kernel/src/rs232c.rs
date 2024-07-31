//! # String writing function by RS232C COM2
//! GPD MicroPC, a tester hardware of the OS, has a RS232C port as COM2.
//! So we use COM2 to log boot progress.
//! ## References
//! * [Serial Ports - OSdev WiKi](https://wiki.osdev.org/Serial_Ports)
//! * [Look RS232](https://www.lookrs232.com/rs232/registers.htm)

use {
    core::{
        cell::OnceCell,
        fmt::{
            self,
            Write,
        },
    },
    crate::x64,
};

mod fifo_control;
mod interrupt_enable;
mod line_control;
mod line_status;
mod modem_control;

/// # Print with a line feed to COM2
/// Usage is the same as [println](https://doc.rust-lang.org/std/macro.println.html)
#[macro_export]
macro_rules! com2_println {
    ($fmt:expr) => ($crate::com2_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::com2_print!(concat!($fmt, "\n"), $($arg)*));
}

/// # Print without a line feed to COM2
/// Usage is the same as [print](https://doc.rust-lang.org/std/macro.print.html)
#[macro_export]
macro_rules! com2_print {
    ($($arg:tt)*) => ($crate::rs232c::com2_print(format_args!($($arg)*)));
}

pub fn com2_print(args: fmt::Arguments) {
    get_com2()
        .write_fmt(args)
        .unwrap()
}

pub fn get_com2() -> &'static mut Com {
    unsafe {
        COM2.get_mut()
            .unwrap()
    }
}

pub fn set_com2(com2: &'static mut Com) {
    unsafe {
        COM2.set(com2)
    }.unwrap();
}

static mut COM2: OnceCell<&'static mut Com> = OnceCell::new();

#[derive(Clone, Copy, Debug)]
pub struct Com {
    port: u16,
}

impl Com {
    #[allow(dead_code)]
    const FREQUENCY: u32 = 115200;
    #[allow(dead_code)]
    const OFFSET_DIVISOR_LATCH_LOW_BYTE: u16 = 0;
    const OFFSET_TRANSMITTER_HOLDING_BUFFER: u16 = 0;
    #[allow(dead_code)]
    const OFFSET_DIVISOR_LATCH_HIGH_BYTE: u16 = 1;
    #[allow(dead_code)]
    const OFFSET_INTERRUPT_ENABLE: u16 = 1;
    #[allow(dead_code)]
    const OFFSET_FIFO_CONTROL: u16 = 2;
    const OFFSET_LINE_CONTROL: u16 = 3;
    #[allow(dead_code)]
    const OFFSET_MODEM_CONTROL: u16 = 4;
    const OFFSET_LINE_STATUS: u16 = 5;

    fn can_send(&self) -> bool {
        self.read_line_status().can_send()
    }

    #[allow(dead_code)]
    fn disable_all_interrupts(&self) {
        self.write_interrupt_enable(self.read_interrupt_enable().disable_all_interrupts());
    }

    fn disable_divisor_latch_access(&self) {
        let line_control: line_control::Register = self.read_line_control();
        if line_control.read_divisor_latch_access() {
            self.write_line_control(line_control.disable_divisor_latch_access());
        }
    }

    #[allow(dead_code)]
    fn enable_divisor_latch_access(&self) {
        let line_control: line_control::Register = self.read_line_control();
        if !line_control.read_divisor_latch_access() {
            self.write_line_control(line_control.enable_divisor_latch_access());
        }
    }

    #[allow(dead_code)]
    fn new(port: u16, baud_rate: u32) -> Self {
        let com = Self {
            port
        };
        com.disable_all_interrupts();
        com.write_baud_rate(baud_rate);
        // 8 bits, no parity, one stop bit
        let word_length: u8 = 8;
        let length_of_stop = line_control::LengthOfStop::One;
        let parity_select = line_control::ParitySelect::No;
        let set_break_enable: bool = false;
        let divisor_latch_access: bool = false;
        com.write_line_control(line_control::Register::create(
            word_length,
            length_of_stop,
            parity_select,
            set_break_enable,
            divisor_latch_access,
        ));
        // Enable FIFO, clear them, with 14-byte threshold
        let enable_fifo: bool = true;
        let clear_receive_fifo: bool = true;
        let clear_transmit_fifo: bool = true;
        let dma_mode_select: bool = false;
        let enable_64byte_fifo: bool = false;
        let interrupt_trigger_level: u8 = 14;
        com.write_fifo_control(fifo_control::Register::create(
            enable_fifo,
            clear_receive_fifo,
            clear_transmit_fifo,
            dma_mode_select,
            enable_64byte_fifo,
            interrupt_trigger_level,
        ));
        // IRQs enabled, RTS/DSR set
        let force_data_terminal_ready: bool = true;
        let force_request_to_send: bool = true;
        let aux_output1: bool = false;
        let aux_output2: bool = true;
        let loopback_mode: bool = false;
        let autoflow_control_enabled: bool = false;
        com.write_modem_control(modem_control::Register::create(
            force_data_terminal_ready,
            force_request_to_send,
            aux_output1,
            aux_output2,
            loopback_mode,
            autoflow_control_enabled,
        ));
        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        let force_data_terminal_ready: bool = true;
        let force_request_to_send: bool = true;
        let aux_output1: bool = true;
        let aux_output2: bool = true;
        let loopback_mode: bool = false;
        let autoflow_control_enabled: bool = false;
        com.write_modem_control(modem_control::Register::create(
            force_data_terminal_ready,
            force_request_to_send,
            aux_output1,
            aux_output2,
            loopback_mode,
            autoflow_control_enabled,
        ));
        com
    }

    #[allow(dead_code)]
    fn port_divisor_latch_high_byte(&self) -> u16 {
        self.port + Self::OFFSET_DIVISOR_LATCH_HIGH_BYTE
    }

    #[allow(dead_code)]
    fn port_divisor_latch_low_byte(&self) -> u16 {
        self.port + Self::OFFSET_DIVISOR_LATCH_LOW_BYTE
    }

    #[allow(dead_code)]
    fn port_fifo_control(&self) -> u16 {
        self.port + Self::OFFSET_FIFO_CONTROL
    }

    #[allow(dead_code)]
    fn port_interrupt_enable(&self) -> u16 {
        self.port + Self::OFFSET_INTERRUPT_ENABLE
    }

    fn port_line_control(&self) -> u16 {
        self.port + Self::OFFSET_LINE_CONTROL
    }

    fn port_line_status(&self) -> u16 {
        self.port + Self::OFFSET_LINE_STATUS
    }

    #[allow(dead_code)]
    fn port_modem_control(&self) -> u16 {
        self.port + Self::OFFSET_MODEM_CONTROL
    }

    fn port_transmitter_holding_buffer(&self) -> u16 {
        self.port + Self::OFFSET_TRANSMITTER_HOLDING_BUFFER
    }

    #[allow(dead_code)]
    fn read_interrupt_enable(&self) -> interrupt_enable::Register {
        self.disable_divisor_latch_access();
        x64::port::inb(self.port_interrupt_enable()).into()
    }

    fn read_line_control(&self) -> line_control::Register {
        x64::port::inb(self.port_line_control()).into()
    }

    fn read_line_status(&self) -> line_status::Register {
        x64::port::inb(self.port_line_status()).into()
    }

    fn send(&self, data: u8) {
        while !self.can_send() {
            x64::pause();
        }
        self.write_transmitter_holding_buffer(data);
    }

    #[allow(dead_code)]
    fn write_baud_rate(&self, baud_rate: u32) {
        let divisor_latch: u32 = Self::FREQUENCY / baud_rate;
        let divisor_latch: u16 = divisor_latch as u16;
        let divisor_latch_low: u16 = divisor_latch & 0x00ff;
        let divisor_latch_low: u8 = divisor_latch_low as u8;
        let divisor_latch_high: u16 = divisor_latch >> 8;
        let divisor_latch_high: u8 = divisor_latch_high as u8;
        self.enable_divisor_latch_access();
        x64::port::outb(self.port_divisor_latch_low_byte(), divisor_latch_low);
        x64::port::outb(self.port_divisor_latch_high_byte(), divisor_latch_high);
    }

    #[allow(dead_code)]
    fn write_fifo_control(&self, register: fifo_control::Register) {
        x64::port::outb(self.port_fifo_control(), register.into());
    }

    #[allow(dead_code)]
    fn write_interrupt_enable(&self, register: interrupt_enable::Register) {
        self.disable_divisor_latch_access();
        x64::port::outb(self.port_interrupt_enable(), register.into());
    }

    fn write_line_control(&self, register: line_control::Register) {
        x64::port::outb(self.port_line_control(), register.into());
    }

    #[allow(dead_code)]
    fn write_modem_control(&self, register: modem_control::Register) {
        x64::port::outb(self.port_modem_control(), register.into());
    }

    fn write_transmitter_holding_buffer(&self, data: u8) {
        self.disable_divisor_latch_access();
        x64::port::outb(self.port_transmitter_holding_buffer(), data);
    }
}

impl fmt::Write for Com {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        string
            .bytes()
            .for_each(|byte| self.send(byte));
        Ok(())
    }
}

