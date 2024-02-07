use crate::{
    rs232c,
    x64,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Argument<'a> {
    com: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>
}

impl<'a> Argument<'a> {
    pub fn new(com: &'a mut rs232c::Com, cpuid: Option<x64::Cpuid>) -> Self {
        Self {
            com,
            cpuid,
        }
    }
}

