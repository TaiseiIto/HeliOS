use crate::asm;

mod line_control;

pub struct Com {
    port: u16,
}

pub fn com2() -> Com {
    Com::new(Com::COM2)
}

impl Com {
    const COM2: u16 = 0x02f8;
    const LINE_CONTROL_OFFSET: u16 = 3;

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

    fn line_control_port(&self) -> u16 {
        self.port + Self::LINE_CONTROL_OFFSET
    }

    fn new(port: u16) -> Self {
        Self {
            port,
        }
    }

    fn read_line_control(&self) -> line_control::Register {
        asm::inb(self.line_control_port()).into()
    }

    fn write_line_control(&self, register: line_control::Register) {
        asm::outb(self.line_control_port(), register.into());
    }
}

