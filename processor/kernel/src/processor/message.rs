use alloc::alloc::Layout;

#[derive(Clone, Debug)]
pub enum Content {
    AllocationRequest(Layout),
    AllocationResponse(*mut u8),
    BootCompleted,
    Char(char),
    DeallocationRequest(*mut u8),
    DeallocationResponse,
    KernelCompleted,
}

impl Content {
    pub fn allocation_request(layout: Layout) -> Self {
        Self::AllocationRequest(layout)
    }

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

