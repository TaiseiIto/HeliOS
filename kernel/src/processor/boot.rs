use {
    alloc::{
        vec::Vec,
        string::String,
    },
    core::{
        fmt,
        ops::Range,
        slice,
    },
};

pub struct Loader {
    program_address_range: Range<usize>,
    stack_address_range: Range<usize>,
}

impl Loader {
    pub fn entry_point(&self) -> usize {
        self.program_address_range.start
    }

    pub fn initialize_stack(&mut self) {
        self.stack_mut()
            .iter_mut()
            .for_each(|byte| *byte = 0)
    }

    pub fn log(&self) -> String {
        let log: Vec<u8> = self.stack()
            .iter()
            .map(|byte| *byte)
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

    fn stack_mut(&self) -> &mut [u8] {
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

