use alloc::alloc::Layout;

#[derive(Clone, Debug)]
pub enum Content {
    BootCompleted,
    Char(char),
    KernelCompleted,
}

impl Content {
    pub fn boot_completed() -> Self {
        Self::BootCompleted
    }

    pub fn char(character: char) -> Self {
        Self::Char(character)
    }

    pub fn kernel_completed() -> Self {
        Self::KernelCompleted
    }
}

