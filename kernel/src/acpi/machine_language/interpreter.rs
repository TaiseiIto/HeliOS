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

    fn match_type(self, other: Self) -> (Self, Self) {
        match (self, other) {
            (Self::Byte(left), Self::Zero) => (Self::Byte(left), Self::Byte(0x00)),
            (Self::Byte(left), Self::One) => (Self::Byte(left), Self::Byte(0x01)),
            (Self::Byte(left), Self::Ones) => (Self::Byte(left), Self::Byte(0xff)),
            (Self::Byte(left), Self::Byte(right)) => (Self::Byte(left), Self::Byte(right)),
            (Self::Byte(left), Self::Word(right)) => (Self::Word(left as u16), Self::Word(right)),
            (Self::Byte(left), Self::DWord(right)) => (Self::DWord(left as u32), Self::DWord(right)),
            (Self::Byte(left), Self::QWord(right)) => (Self::QWord(left as u64), Self::QWord(right)),
            (Self::Word(left), Self::Zero) => (Self::Word(left), Self::Word(0x0000)),
            (Self::Word(left), Self::One) => (Self::Word(left), Self::Word(0x0001)),
            (Self::Word(left), Self::Ones) => (Self::Word(left), Self::Word(0xffff)),
            (Self::Word(left), Self::Byte(right)) => (Self::Word(left), Self::Word(right as u16)),
            (Self::Word(left), Self::Word(right)) => (Self::Word(left), Self::Word(right)),
            (Self::Word(left), Self::DWord(right)) => (Self::DWord(left as u32), Self::DWord(right)),
            (Self::Word(left), Self::QWord(right)) => (Self::QWord(left as u64), Self::QWord(right)),
            (Self::DWord(left), Self::Zero) => (Self::DWord(left), Self::DWord(0x00000000)),
            (Self::DWord(left), Self::One) => (Self::DWord(left), Self::DWord(0x00000001)),
            (Self::DWord(left), Self::Ones) => (Self::DWord(left), Self::DWord(0xffffffff)),
            (Self::DWord(left), Self::Byte(right)) => (Self::DWord(left), Self::DWord(right as u32)),
            (Self::DWord(left), Self::Word(right)) => (Self::DWord(left), Self::DWord(right as u32)),
            (Self::DWord(left), Self::DWord(right)) => (Self::DWord(left), Self::DWord(right)),
            (Self::DWord(left), Self::QWord(right)) => (Self::QWord(left as u64), Self::QWord(right)),
            (Self::QWord(left), Self::Zero) => (Self::QWord(left), Self::QWord(0x0000000000000000)),
            (Self::QWord(left), Self::One) => (Self::QWord(left), Self::QWord(0x0000000000000001)),
            (Self::QWord(left), Self::Ones) => (Self::QWord(left), Self::QWord(0xffffffffffffffff)),
            (Self::QWord(left), Self::Byte(right)) => (Self::QWord(left), Self::QWord(right as u64)),
            (Self::QWord(left), Self::Word(right)) => (Self::QWord(left), Self::QWord(right as u64)),
            (Self::QWord(left), Self::DWord(right)) => (Self::QWord(left), Self::QWord(right as u64)),
            (Self::QWord(left), Self::QWord(right)) => (Self::QWord(left), Self::QWord(right)),
            _ => unimplemented!("self = {:#x?}\nother = {:#x?}", self, other),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self.match_type(other) {
            (Self::Byte(left), Self::Byte(right)) => Self::Byte(left.wrapping_add(right)),
            (Self::Word(left), Self::Word(right)) => Self::Word(left.wrapping_add(right)),
            (Self::DWord(left), Self::DWord(right)) => Self::DWord(left.wrapping_add(right)),
            (Self::QWord(left), Self::QWord(right)) => Self::QWord(left.wrapping_add(right)),
            _ => unimplemented!("self = {:#x?}\nother = {:#x?}", self, other),
        }
    }
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::Bool(value) => *value,
            Value::Buffer(buffer) => buffer
                .iter()
                .any(|byte| *byte != 0),
            Value::Byte(byte) => *byte != 0,
            Value::Char(character) => (*character as u32) != 0,
            Value::DWord(dword) => *dword != 0,
            Value::One => true,
            Value::Ones => true,
            Value::Package(package) => package
                .iter()
                .any(|value| value.into()),
            Value::QWord(qword) => *qword != 0,
            Value::Revision => unreachable!(),
            Value::String(string) => !string.is_empty(),
            Value::Word(word) => *word != 0,
            Value::Zero => false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StackFrame {
    arguments: [Option<Value>; 0x07],
    locals: [Option<Value>; 0x08],
    named_locals: BTreeMap<String, Value>,
    return_value: Option<Value>,
}

impl StackFrame {
    pub fn read_argument(&self, index: usize) -> Option<Value> {
        self.arguments[index].clone()
    }

    pub fn read_local(&self, index: usize) -> Option<Value> {
        self.locals[index].clone()
    }

    pub fn read_return(&self) -> Option<&Value> {
        self.return_value
            .as_ref()
    }

    pub fn set_arguments(self, arguments: Vec<Value>) -> Self {
        let Self {
            arguments,
            locals,
            named_locals,
            return_value,
        } = self;
        let arguments: Vec<Option<Value>> = arguments
            .into_iter()
            .map(Some)
            .chain(arguments
                .as_slice()
                .iter()
                .map(|_| None))
            .take(arguments
                .as_slice()
                .len())
            .collect();
        let arguments = arguments
            .try_into()
            .unwrap();
        Self {
            arguments,
            locals,
            named_locals,
            return_value,
        }
    }

    pub fn write_argument(&mut self, index: usize, value: Value) -> Value {
        self.arguments[index] = Some(value.clone());
        value
    }

    pub fn write_local(&mut self, index; usize, value: Value) -> Value {
        self.locals[index] = Some(value.clone());
        value
    }
}

pub trait Evaluator {
    fn evaluate(&self, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Option<Value>;
}

pub trait Holder {
    fn hold(&self, value: Value, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Value;
}

