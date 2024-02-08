use crate::{
    efi,
    interrupt,
    memory,
    rs232c,
    x64,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Argument<'a> {
    com: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    gdt: memory::segment::descriptor::Table,
    idt: interrupt::descriptor::Table,
    paging: memory::Paging,
}

impl<'a> Argument<'a> {
    pub fn new(com: &'a mut rs232c::Com, cpuid: Option<x64::Cpuid>, efi_system_table: &'a mut efi::SystemTable<'a>, gdt: memory::segment::descriptor::Table, idt: interrupt::descriptor::Table, paging: memory::Paging) -> Self {
        Self {
            com,
            cpuid,
            efi_system_table,
            gdt,
            idt,
            paging,
        }
    }
}

