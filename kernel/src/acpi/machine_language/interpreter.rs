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

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Buffer(Vec<u8>),
    Byte(u8),
    Char(char),
    DWord(u32),
    One,
    Ones,
    Package(Vec<Self>),
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
                .chain(second)
                .collect()),
            (Self::Package(first), Self::Package(second)) => Self::Package(first
                .into_iter()
                .chain(second)
                .collect()),
            (Self::String(first), Self::String(second)) => Self::String(first + &second),
            _ => unimplemented!(),
        }
    }

    pub fn get_byte(&self) -> Option<u8> {
        match self {
            Self::Byte(byte) => Some(*byte),
            Self::Zero => Some(0x00),
            Self::One => Some(0x01),
            Self::Ones => Some(0xff),
            _ => None,
        }
    }

    pub fn get_element(&self, index: usize) -> Option<Self> {
        match self {
            Self::Buffer(bytes) => bytes
                .get(index)
                .cloned()
                .map(Self::Byte),
            Self::Package(elements) => elements
                .get(index)
                .cloned(),
            Self::String(characters) => characters
                .chars()
                .nth(index)
                .map(Self::Char),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct StackFrame {
    argument_values: [Option<Value>; 0x07],
    local_values: [Option<Value>; 0x08],
    named_local_values: BTreeMap<String, Value>,
    return_value: Option<Value>,
}

impl StackFrame {
    pub fn argument_value(&self, index: usize) -> Option<Value> {
        self.argument_values[index].clone()
    }

    pub fn local_value(&self, index: usize) -> Option<Value> {
        self.local_values[index].clone()
    }

    pub fn set_arguments(self, arguments: Vec<Value>) -> Self {
        let Self {
            argument_values,
            local_values,
            named_local_values,
            return_value,
        } = self;
        let argument_values: Vec<Option<Value>> = arguments
            .into_iter()
            .map(Some)
            .chain(argument_values
                .as_slice()
                .iter()
                .map(|_| None))
            .take(argument_values
                .as_slice()
                .len())
            .collect();
        let argument_values = argument_values
            .try_into()
            .unwrap();
        Self {
            argument_values,
            local_values,
            named_local_values,
            return_value,
        }
    }
}

pub trait Evaluator {
    fn evaluate(&self, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Option<Value>;
}

