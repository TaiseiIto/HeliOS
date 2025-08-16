pub mod boot;

use {
    alloc::vec::Vec,
    core::ops::Range,
    crate::efi,
};

pub fn prepare(directory_tree: &efi::file::system::Tree, boot_loader: &str, kernel: &str, boot_loader_pages: Range<efi::memory::PhysicalAddress>) -> (boot::Loader, Vec<u8>) {
    let boot_loader: Vec<u8> = directory_tree
        .get(boot_loader)
        .unwrap()
        .read();
    let boot_loader = boot::Loader::new(&boot_loader, boot_loader_pages);
    let kernel: Vec<u8> = directory_tree
        .get(kernel)
        .unwrap()
        .read();
    (boot_loader, kernel)
}

