//! # Simple File System Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.4 Simple File System Protocol

use {
    super::super::{
        super::{null, Guid, Status, SystemTable, Void},
        file,
    },
    alloc::vec::Vec,
};

/// # EFI_SIMPLE_FILE_SYSTEM_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.4 Simple File System Protocol
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    revision: u64,
    open_volume: OpenVolume,
}

impl Protocol {
    pub fn get() -> &'static Self {
        let guid = Guid::new(
            0x964e5b22,
            0x6459,
            0x11d2,
            [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
        );
        let registration: &Void = null();
        let protocol: &Void = SystemTable::get()
            .locate_protocol(registration, guid)
            .unwrap();
        let protocol: *const Void = protocol as *const Void;
        let protocol: *const Protocol = protocol as *const Protocol;
        unsafe { &*protocol }
    }

    pub fn tree(&self) -> Tree {
        self.root().into()
    }

    fn root(&self) -> file::Node {
        let mut root: &file::Protocol = null();
        (self.open_volume)(self, &mut root).result().unwrap();
        root.into()
    }
}

/// # EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.4 Simple File System Protocol
type OpenVolume = extern "efiapi" fn(/* This */ &Protocol, &mut &file::Protocol) -> Status;

#[derive(Debug)]
pub struct Tree<'a> {
    node: file::Node<'a>,
    children: Vec<Self>,
}

impl<'a> Tree<'a> {
    pub fn get(&self, path: &'a str) -> Option<&file::Node<'a>> {
        self.get_by_iter(path.split('/'))
    }

    fn get_by_iter<I>(&self, mut path: I) -> Option<&file::Node<'a>>
    where
        I: Iterator<Item = &'a str>,
    {
        match path.next() {
            Some(name) => self
                .children
                .iter()
                .find(|child| child.name() == name)
                .and_then(|child| child.get_by_iter(path)),
            None => Some(&self.node),
        }
    }

    fn name(&self) -> &str {
        self.node.name()
    }
}

impl<'a> From<file::Node<'a>> for Tree<'a> {
    fn from(node: file::Node<'a>) -> Self {
        let children: Vec<Self> = node
            .clone()
            .filter(|child| [".", ".."].into_iter().all(|name| child.name() != name))
            .map(|child| child.into())
            .collect();
        Self { node, children }
    }
}
