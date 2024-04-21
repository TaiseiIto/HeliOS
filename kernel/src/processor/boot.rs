use {
    alloc::{
        vec::Vec,
        string::String,
    },
    core::{
        fmt,
        mem,
        ops::Range,
        slice,
    },
    crate::x64,
};

pub struct Loader {
    program_address_range: Range<usize>,
    stack_address_range: Range<usize>,
}

impl Loader {
    pub fn entry_point(&self) -> usize {
        self.program_address_range.start
    }

    pub fn initialize(&mut self) {
        self.initialize_stack();
        self.set_arguments();
    }

    pub fn log(&self) -> String {
        let log: Vec<u8> = self.stack()
            .iter()
            .copied()
            .take_while(|byte| *byte != 0)
            .collect();
        String::from_utf8(log).unwrap()
    }

    pub fn program(&self) -> &[u8] {
        let start: *const u8 = self.program_address_range.start as *const u8;
        let length: usize = self.program_address_range.end - self.program_address_range.start;
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }

    pub fn stack(&self) -> &[u8] {
        let start: *const u8 = self.stack_address_range.start as *const u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }

    fn arguments_mut(&mut self) -> &mut Arguments {
        let arguments: usize = self.program_address_range.end - mem::size_of::<Arguments>();
        let arguments: *mut Arguments = arguments as *mut Arguments;
        unsafe {
            &mut *arguments
        }
    }

    fn initialize_stack(&mut self) {
        self.stack_mut()
            .iter_mut()
            .for_each(|byte| *byte = 0)
    }

    fn set_arguments(&mut self) {
        *self.arguments_mut() = Arguments::new();
    }

    fn stack_mut(&mut self) -> &mut [u8] {
        let start: *mut u8 = self.stack_address_range.start as *mut u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
        unsafe {
            slice::from_raw_parts_mut(start, length)
        }
    }
}

impl fmt::Debug for Loader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Loader")
            .field("program", &self.program())
            .field("stack", &self.stack())
            .finish()
    }
}

#[repr(packed)]
struct Arguments {
    cr3: u64,
}

impl Arguments {
    pub fn new() -> Self {
        let cr3: u64 = x64::control::Register3::get().into();
        Self {
            cr3,
        }
    }
}

