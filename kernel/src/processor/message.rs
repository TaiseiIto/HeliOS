use super::Controller;

#[derive(Clone, Debug)]
pub enum Content {
    BootCompleted,
    Char(char),
    HpetInterrupt,
    KernelCompleted,
}

impl Content {
    pub fn hpet_interrupt() -> Self {
        Self::HpetInterrupt
    }

    pub fn process(self, controller: &mut Controller) {
        match self {
            Self::BootCompleted => controller.boot_complete(),
            Self::Char(character) => controller.receive_character(character),
            Self::HpetInterrupt => unimplemented!(),
            Self::KernelCompleted => controller.kernel_complete(),
        }
    }
}

