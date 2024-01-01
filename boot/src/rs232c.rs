use crate::asm;

mod interrupt_enable;
mod line_control;

pub struct Com {
    port: u16,
}

const COM2: u16 = 0x02f8;

pub fn com2() -> Com {
    let port: u16 = COM2;
    Com {
        port,
    }
}

// https://www.lookrs232.com/rs232/registers.htm
impl Com {
    const OFFSET_INTERRUPT_ENABLE: u16 = 1;
    const OFFSET_LINE_CONTROL: u16 = 3;

    fn disable_all_interrupts(&self) {
        self.write_interrupt_enable(self.read_interrupt_enable().disable_all_interrupts());
    }

    fn disable_divisor_latch_access_bit(&self) {
        let line_control: line_control::Register = self.read_line_control();
        if line_control.read_divisor_latch_access_bit() {
            self.write_line_control(line_control.disable_divisor_latch_access_bit());
        }
    }

    fn enable_divisor_latch_access_bit(&self) {
        let line_control: line_control::Register = self.read_line_control();
        if !line_control.read_divisor_latch_access_bit() {
            self.write_line_control(line_control.enable_divisor_latch_access_bit());
        }
    }

    fn port_interrupt_enable(&self) -> u16 {
        self.port + Self::OFFSET_INTERRUPT_ENABLE
    }

    fn port_line_control(&self) -> u16 {
        self.port + Self::OFFSET_LINE_CONTROL
    }

    fn read_interrupt_enable(&self) -> interrupt_enable::Register {
        self.disable_divisor_latch_access_bit();
        asm::inb(self.port_interrupt_enable()).into()
    }

    fn read_line_control(&self) -> line_control::Register {
        asm::inb(self.port_line_control()).into()
    }

    fn write_interrupt_enable(&self, register: interrupt_enable::Register) {
        self.disable_divisor_latch_access_bit();
        asm::outb(self.port_interrupt_enable(), register.into());
    }

    fn write_line_control(&self, register: line_control::Register) {
        asm::outb(self.port_line_control(), register.into());
    }
}

