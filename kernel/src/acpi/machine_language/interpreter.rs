use {
    alloc::{
        string::String,
        vec::Vec,
    },
    super::{
        name,
        reference,
    },
};

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Buffer(Vec<u8>),
    Byte(u8),
    DWord(u32),
    One,
    Ones,
    QWord(u64),
    Revision,
    String(String),
    Word(u16),
    Zero,
}

impl Data {
    pub fn concatenate(self, other: Self) -> Self {
        match (self, other) {
            (Self::Byte(low), Self::Byte(high)) => Self::Word(((high as u16) << u8::BITS) + (low as u16)),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct StackFrame {
    args: [Option<Data>; 0x07],
    locals: [Option<Data>; 0x08],
}

pub trait Evaluator {
    fn evaluate(&self, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Option<Data>;
}

