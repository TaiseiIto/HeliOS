use {
    alloc::{
        collections::BTreeMap,
        string::String,
        vec::Vec,
    },
    core::{
        cmp::Ordering,
        iter,
        ops::Add,
    },
    super::{
        name,
        reference,
    },
};

#[derive(Clone, Debug, Eq, Ord, PartialEq)]
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

    /// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 19.6.124 SizeOf (Get Data Object Size)
    pub fn size(&self) -> Self {
        let size: usize = match self {
            Self::Buffer(buffer) => buffer.len(),
            Self::String(string) => string.len(),
            Self::Package(package) => package.len(),
            value => unreachable!("value = {:#x?}", value),
        };
        let size: u64 = size as u64;
        Self::QWord(size)
    }

    fn match_type(&self, other: &Self) -> (Self, Self) {
        match (self, other) {
            (Self::Bool(left), Self::Bool(right)) => (Self::Bool(*left), Self::Bool(*right)),
            (Self::Buffer(left), Self::Buffer(right)) => (Self::Buffer(left.clone()), Self::Buffer(right.clone())),
            (Self::Byte(left), Self::Byte(right)) => (Self::Byte(*left), Self::Byte(*right)),
            (Self::Byte(left), Self::DWord(right)) => (Self::DWord(*left as u32), Self::DWord(*right)),
            (Self::Byte(left), Self::One) => (Self::Byte(*left), Self::Byte(0x01)),
            (Self::Byte(left), Self::Ones) => (Self::Byte(*left), Self::Byte(0xff)),
            (Self::Byte(left), Self::QWord(right)) => (Self::QWord(*left as u64), Self::QWord(*right)),
            (Self::Byte(left), Self::Word(right)) => (Self::Word(*left as u16), Self::Word(*right)),
            (Self::Byte(left), Self::Zero) => (Self::Byte(*left), Self::Byte(0x00)),
            (Self::Char(left), Self::Char(right)) => (Self::Char(*left), Self::Char(*right)),
            (Self::DWord(left), Self::Byte(right)) => (Self::DWord(*left), Self::DWord(*right as u32)),
            (Self::DWord(left), Self::DWord(right)) => (Self::DWord(*left), Self::DWord(*right)),
            (Self::DWord(left), Self::One) => (Self::DWord(*left), Self::DWord(0x00000001)),
            (Self::DWord(left), Self::Ones) => (Self::DWord(*left), Self::DWord(0xffffffff)),
            (Self::DWord(left), Self::QWord(right)) => (Self::QWord(*left as u64), Self::QWord(*right)),
            (Self::DWord(left), Self::Word(right)) => (Self::DWord(*left), Self::DWord(*right as u32)),
            (Self::DWord(left), Self::Zero) => (Self::DWord(*left), Self::DWord(0x00000000)),
            (Self::One, Self::Byte(right)) => (Self::Byte(0x01), Self::Byte(*right)),
            (Self::One, Self::DWord(right)) => (Self::DWord(0x00000001), Self::DWord(*right)),
            (Self::One, Self::One) => (Self::One, Self::One),
            (Self::One, Self::Ones) => (Self::One, Self::Ones),
            (Self::One, Self::QWord(right)) => (Self::QWord(0x0000000000000001), Self::QWord(*right)),
            (Self::One, Self::Word(right)) => (Self::Word(0x0001), Self::Word(*right)),
            (Self::One, Self::Zero) => (Self::One, Self::Zero),
            (Self::Ones, Self::Byte(right)) => (Self::Byte(0xff), Self::Byte(*right)),
            (Self::Ones, Self::DWord(right)) => (Self::DWord(0xffffffff), Self::DWord(*right)),
            (Self::Ones, Self::One) => (Self::Ones, Self::One),
            (Self::Ones, Self::Ones) => (Self::Ones, Self::Ones),
            (Self::Ones, Self::QWord(right)) => (Self::QWord(0xffffffffffffffff), Self::QWord(*right)),
            (Self::Ones, Self::Word(right)) => (Self::Word(0xffff), Self::Word(*right)),
            (Self::Ones, Self::Zero) => (Self::Ones, Self::Zero),
            (Self::Package(left), Self::Package(right)) => (Self::Package(left.clone()), Self::Package(right.clone())),
            (Self::QWord(left), Self::Byte(right)) => (Self::QWord(*left), Self::QWord(*right as u64)),
            (Self::QWord(left), Self::DWord(right)) => (Self::QWord(*left), Self::QWord(*right as u64)),
            (Self::QWord(left), Self::One) => (Self::QWord(*left), Self::QWord(0x0000000000000001)),
            (Self::QWord(left), Self::Ones) => (Self::QWord(*left), Self::QWord(0xffffffffffffffff)),
            (Self::QWord(left), Self::QWord(right)) => (Self::QWord(*left), Self::QWord(*right)),
            (Self::QWord(left), Self::Word(right)) => (Self::QWord(*left), Self::QWord(*right as u64)),
            (Self::QWord(left), Self::Zero) => (Self::QWord(*left), Self::QWord(0x0000000000000000)),
            (Self::Revision, Self::Revision) => (Self::Revision, Self::Revision),
            (Self::String(left), Self::String(right)) => (Self::String(left.clone()), Self::String(right.clone())),
            (Self::Word(left), Self::Byte(right)) => (Self::Word(*left), Self::Word(*right as u16)),
            (Self::Word(left), Self::DWord(right)) => (Self::DWord(*left as u32), Self::DWord(*right)),
            (Self::Word(left), Self::One) => (Self::Word(*left), Self::Word(0x0001)),
            (Self::Word(left), Self::Ones) => (Self::Word(*left), Self::Word(0xffff)),
            (Self::Word(left), Self::QWord(right)) => (Self::QWord(*left as u64), Self::QWord(*right)),
            (Self::Word(left), Self::Word(right)) => (Self::Word(*left), Self::Word(*right)),
            (Self::Word(left), Self::Zero) => (Self::Word(*left), Self::Word(0x0000)),
            (Self::Zero, Self::Byte(right)) => (Self::Byte(0x00), Self::Byte(*right)),
            (Self::Zero, Self::DWord(right)) => (Self::DWord(0x00000000), Self::DWord(*right)),
            (Self::Zero, Self::One) => (Self::Zero, Self::One),
            (Self::Zero, Self::Ones) => (Self::Zero, Self::Ones),
            (Self::Zero, Self::QWord(right)) => (Self::QWord(0x0000000000000000), Self::QWord(*right)),
            (Self::Zero, Self::Word(right)) => (Self::Word(0x0000), Self::Word(*right)),
            (Self::Zero, Self::Zero) => (Self::Zero, Self::Zero),
            (left, right)  => unimplemented!("left = {:#x?}\nright = {:#x?}", left, right),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self.match_type(&other) {
            (Self::Byte(left), Self::Byte(right)) => Self::Byte(left.wrapping_add(right)),
            (Self::Word(left), Self::Word(right)) => Self::Word(left.wrapping_add(right)),
            (Self::DWord(left), Self::DWord(right)) => Self::DWord(left.wrapping_add(right)),
            (Self::QWord(left), Self::QWord(right)) => Self::QWord(left.wrapping_add(right)),
            (left, right) => unimplemented!("left = {:#x?}\nright = {:#x?}", left, right),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Self::Buffer(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self::Char(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::Word(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::DWord(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::QWord(value)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.match_type(other) {
            (Self::Byte(left), Self::Byte(right)) => left.partial_cmp(&right),
            (Self::Word(left), Self::Word(right)) => left.partial_cmp(&right),
            (Self::DWord(left), Self::DWord(right)) => left.partial_cmp(&right),
            (Self::QWord(left), Self::QWord(right)) => left.partial_cmp(&right),
            (left, right) => unimplemented!("left = {:#x?}\nright = {:#x?}", left, right),
        }
    }
}

impl From<&Value> for Vec<u8> {
    fn from(value: &Value) -> Self {
        match value {
            Value::Buffer(buffer) => buffer.clone(),
            value => unimplemented!("value = {:#x?}", value),
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

impl From<&Value> for usize {
    fn from(value: &Value) -> Self {
        match value {
            Value::Byte(byte) => *byte as Self,
            Value::DWord(dword) => *dword as Self,
            Value::One => 1,
            Value::Ones => usize::MAX,
            Value::QWord(qword) => *qword as Self,
            Value::Word(word) => *word as Self,
            Value::Zero => 0,
            value => unimplemented!("value = {:#x?}", value),
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
    pub fn add_named_local(&mut self, name: &str, value: Value) {
        self.named_locals.insert(name.into(), value);
    }

    pub fn has_local(&self, name: &str) -> bool {
        self.named_locals.contains_key(name)
    }

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

    pub fn set_arguments(self, new_arguments: Vec<Value>) -> Self {
        let Self {
            arguments,
            locals,
            named_locals,
            return_value,
        } = self;
        let num_of_arguments: usize = arguments
            .as_slice()
            .len();
        let arguments: Vec<Option<Value>> = new_arguments
            .into_iter()
            .map(Some)
            .chain(iter::repeat(None).take(num_of_arguments))
            .take(num_of_arguments)
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

    pub fn write_local(&mut self, index: usize, value: Value) -> Value {
        self.locals[index] = Some(value.clone());
        value
    }

    pub fn write_named_local(&mut self, name: &str, value: Value) -> Option<Value> {
        self.named_locals
            .get_mut(name)
            .map(|named_local| {
                let (_named_local, value): (Value, Value) = named_local
                    .match_type(value);
                *named_local = value.clone();
                value
            })
    }

    pub fn write_return(&mut self, value: Value) -> Value {
        self.return_value = Some(value.clone());
        value
    }
}

pub trait Evaluator {
    fn evaluate(&self, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Option<Value>;
}

pub trait Holder {
    fn hold(&self, value: Value, stack_frame: &mut StackFrame, root: &reference::Node, current: &name::Path) -> Value;
}

