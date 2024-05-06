use {
    alloc::alloc::Layout,
    super::Controller,
};

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
    pub fn process(self, controller: &mut Controller) {
        match self {
            Self::AllocationRequest(layout) => unimplemented!(),
            Self::AllocationResponse(address) => unimplemented!(),
            Self::BootCompleted => controller.boot_complete(),
            Self::Char(character) => controller.receive_character(character),
            Self::DeallocationRequest(address) => unimplemented!(),
            Self::DeallocationResponse => unimplemented!(),
            Self::KernelCompleted => controller.kernel_complete(),
        }
    }
}

