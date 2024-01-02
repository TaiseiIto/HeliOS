use crate::asm;

mod interrupt_enable;
mod line_control;

pub struct Com {
    port: u16,
}

const BAUD_RATE: u32 = 9600;
const COM2: u16 = 0x02f8;

pub fn com2() -> Com {
    Com::new(COM2)
}

// https://www.lookrs232.com/rs232/registers.htm
impl Com {
    const FREQUENCY: u32 = 115200;
    const OFFSET_DIVISOR_LATCH_HIGH_BYTE: u16 = 0;
    const OFFSET_DIVISOR_LATCH_LOW_BYTE: u16 = 0;
    const OFFSET_INTERRUPT_ENABLE: u16 = 1;
    const OFFSET_LINE_CONTROL: u16 = 3;

    fn disable_all_interrupts(&self) {
        self.write_interrupt_enable(self.read_interrupt_enable().disable_all_interrupts());
    }

    fn disable_divisor_latch_access(&self) {
        let line_control: line_control::Register = self.read_line_control();
        if line_control.read_divisor_latch_access() {
            self.write_line_control(line_control.disable_divisor_latch_access());
        }
    }

    fn enable_divisor_latch_access(&self) {
        let line_control: line_control::Register = self.read_line_control();
        if !line_control.read_divisor_latch_access() {
            self.write_line_control(line_control.enable_divisor_latch_access());
        }
    }

    fn new(port: u16) -> Self {
        let com = Self {
            port
        };
        com.disable_all_interrupts();
        com.write_baud_rate(BAUD_RATE);
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
        com
    }

    fn port_divisor_latch_high_byte(&self) -> u16 {
        self.port + Self::OFFSET_DIVISOR_LATCH_HIGH_BYTE
    }

    fn port_divisor_latch_low_byte(&self) -> u16 {
        self.port + Self::OFFSET_DIVISOR_LATCH_LOW_BYTE
    }

    fn port_interrupt_enable(&self) -> u16 {
        self.port + Self::OFFSET_INTERRUPT_ENABLE
    }

    fn port_line_control(&self) -> u16 {
        self.port + Self::OFFSET_LINE_CONTROL
    }

    fn read_interrupt_enable(&self) -> interrupt_enable::Register {
        self.disable_divisor_latch_access();
        asm::inb(self.port_interrupt_enable()).into()
    }

    fn read_line_control(&self) -> line_control::Register {
        asm::inb(self.port_line_control()).into()
    }

    fn write_baud_rate(&self, baud_rate: u32) {
        let divisor_latch: u32 = Self::FREQUENCY / baud_rate;
        let divisor_latch: u16 = divisor_latch as u16;
        let divisor_latch_low: u16 = divisor_latch & 0x00ff;
        let divisor_latch_low: u8 = divisor_latch_low as u8;
        let divisor_latch_high: u16 = divisor_latch >> 8;
        let divisor_latch_high: u8 = divisor_latch_high as u8;
        self.enable_divisor_latch_access();
        asm::outb(self.port_divisor_latch_low_byte(), divisor_latch_low);
        asm::outb(self.port_divisor_latch_high_byte(), divisor_latch_high);
    }

    fn write_interrupt_enable(&self, register: interrupt_enable::Register) {
        self.disable_divisor_latch_access();
        asm::outb(self.port_interrupt_enable(), register.into());
    }

    fn write_line_control(&self, register: line_control::Register) {
        asm::outb(self.port_line_control(), register.into());
    }
}

