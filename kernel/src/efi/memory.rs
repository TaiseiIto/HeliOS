use {
    alloc::vec::Vec,
    core::ops::Range,
    crate::memory,
    super::Void,
};

/// # EFI_ALLOCATE_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
#[derive(Debug)]
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    #[allow(dead_code)]
    AllocateMaxAddress,
    #[allow(dead_code)]
    AllocateAddress,
    #[allow(dead_code)]
    Max,
}

/// # EFI_MEMORY_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
#[derive(Clone, Debug)]
#[repr(u32)]
pub enum Type {
    #[allow(dead_code)]
    ReservedMemory,
    #[allow(dead_code)]
    LoaderCode,
    LoaderData,
    #[allow(dead_code)]
    BootServicesCode,
    #[allow(dead_code)]
    BootServicesData,
    #[allow(dead_code)]
    RuntimeServicesCode,
    #[allow(dead_code)]
    RuntimeServicesData,
    #[allow(dead_code)]
    ConventionalMemory,
    #[allow(dead_code)]
    UnusableMemory,
    #[allow(dead_code)]
    ACPIReclaimMemory,
    #[allow(dead_code)]
    ACPIMemoryNVS,
    #[allow(dead_code)]
    MemoryMappedIO,
    #[allow(dead_code)]
    MemoryMappedIOPortSpace,
    #[allow(dead_code)]
    PalCode,
    #[allow(dead_code)]
    PersistentMemory,
    #[allow(dead_code)]
    UnacceptedMemory,
    #[allow(dead_code)]
    MaxMemory,
}

impl Type {
    fn is_available(&self) -> bool {
        matches!(self, Self::ConventionalMemory)
    }
}

/// # EFI_PHYSICAL_ADDRESS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
pub type PhysicalAddress = u64;

impl From<&Void> for PhysicalAddress {
    fn from(void: &Void) -> Self {
        let void: *const Void = void as *const Void;
        void as Self
    }
}

/// # EFI_MEMORY_DESCRIPTOR
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Descriptor {
    memory_type: Type,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

impl Descriptor {
    pub fn is_available(&self) -> bool {
        self.memory_type.is_available()
    }

    pub fn physical_start(&self) -> usize {
        self.physical_start as usize
    }

    pub fn physical_end(&self) -> usize {
        self.physical_start() + (self.number_of_pages as usize) * memory::page::SIZE
    }

    pub fn physical_range(&self) -> Range<usize> {
        self.physical_start()..self.physical_end()
    }
}

#[derive(Debug)]
pub struct Map {
    descriptors: Vec<u8>,
    descriptor_size: usize,
    #[allow(dead_code)]
    descriptor_version: u32,
    key: usize,
}

impl Map {
    pub fn get_descriptor(&self, index: usize) -> Option<&Descriptor> {
        (index < self.descriptors.len() / self.descriptor_size)
            .then(|| {
                let offset: usize = index * self.descriptor_size;
                let descriptor: &u8 = &self.descriptors[offset];
                let descriptor: *const u8 = descriptor as *const u8;
                let descriptor: *const Descriptor = descriptor as *const Descriptor;
                unsafe {
                    &*descriptor
                }
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Descriptor> {
        (0..)
            .map_while(|index| self.get_descriptor(index))
    }

    pub fn new(descriptors: Vec<u8>, descriptor_size: usize, descriptor_version: u32, key: usize) -> Self {
        Self {
            descriptors,
            descriptor_size,
            descriptor_version,
            key,
        }
    }

    pub fn key(&self) -> usize {
        self.key
    }
}

impl From<Map> for Vec<Descriptor> {
    fn from(map: Map) -> Vec<Descriptor> {
        map.descriptors
            .chunks(map.descriptor_size)
            .map(|descriptor| {
                let descriptor: *const [u8] = descriptor as *const [u8];
                let descriptor: *const Descriptor = descriptor as *const Descriptor;
                unsafe {
                    descriptor.read_volatile()
                }
            })
            .collect()
    }
}

/// # EFI_VIRTUAL_ADDRESS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
pub type VirtualAddress = u64;

