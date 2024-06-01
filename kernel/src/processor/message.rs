use super::Controller;

#[derive(Clone, Debug)]
pub enum Content {
    BootCompleted,
    Char(char),
    KernelCompleted,
}

impl Content {
    pub fn process(self, controller: &mut Controller) {
        match self {
            Self::BootCompleted => controller.boot_complete(),
            Self::Char(character) => controller.receive_character(character),
            Self::KernelCompleted => controller.kernel_complete(),
        }
    }
}

