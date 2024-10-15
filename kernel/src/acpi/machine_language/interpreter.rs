use {
    alloc::{
        collections::BTreeMap,
        string::String,
        vec::Vec,
    },
    super::{
        name,
        reference,
    },
};

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Buffer(Vec<u8>),
    Byte(u8),
    Char(char),
    DWord(u32),
    One,
    Ones,
    QWord(u64),
    Revision,
    String(String),
    Word(u16),
    Zero,
}

impl Value {
    pub fn concatenate(self, other: Self) -> Self {
        match (self, other) {
            (Self::Byte(low), Self::Byte(high)) => Self::Word((low as u16) + ((high as u16) << u8::BITS)),
            (Self::Word(low), Self::Word(high)) => Self::DWord((low as u32) + ((high as u32) << u16::BITS)),
            (Self::DWord(low), Self::DWord(high)) => Self::QWord((low as u64) + ((high as u64) << u32::BITS)),
            (Self::Buffer(first), Self::Buffer(second)) => Self::Buffer(first
                .into_iter()
                .chain(second.into_iter())
                .collect()),
            (Self::String(first), Self::String(second)) => Self::String(first + &second),
            _ => unimplemented!(),
        }
    }

    pub fn get_byte(&self) -> Option<u8> {
        match self {
            Self::Byte(byte) => Some(*byte),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct StackFrame {
    args: [Option<Value>; 0x07],
    locals: [Option<Value>; 0x08],
    named_locals: BTreeMap<String, Value>,
    return_value: Option<Value>,
}

pub trait Evaluator {
    fn evaluate(&self, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Option<Value>;
}

