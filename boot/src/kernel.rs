use crate::{
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
    gdt: memory::segment::descriptor::Table,
    idt: interrupt::descriptor::Table,
}

impl<'a> Argument<'a> {
    pub fn new(com: &'a mut rs232c::Com, cpuid: Option<x64::Cpuid>, gdt: memory::segment::descriptor::Table, idt: interrupt::descriptor::Table) -> Self {
        Self {
            com,
            cpuid,
            gdt,
            idt,
        }
    }
}

