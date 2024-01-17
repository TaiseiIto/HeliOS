mod pages;

pub use pages::Pages;

use {
    alloc::vec::Vec,
    super::Void,
};

/// # EFI_ALLOCATE_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    Max,
}

/// # EFI_MEMORY_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
#[allow(dead_code)]
#[derive(Clone, Debug)]
#[repr(u32)]
pub enum Type {
    ReservedMemory,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemory,
    MaxMemory,
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

#[derive(Debug)]
pub struct Map {
    descriptors: Vec<u8>,
    descriptor_size: usize,
    #[allow(dead_code)]
    descriptor_version: u32,
    key: usize,
}

impl Map {
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
                let descriptor: &Descriptor = unsafe {
                    &*descriptor
                };
                descriptor.clone()
            })
            .collect()
    }
}

/// # EFI_VIRTUAL_ADDRESS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
pub type VirtualAddress = u64;

