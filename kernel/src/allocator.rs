//! # A memory allocator

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell::OnceCell,
        mem,
        ops::Range,
    },
    crate::{
        com2_print,
        com2_println,
        memory,
    },
};

#[global_allocator]
static mut ALLOCATOR: Allocator<'static> = Allocator {
    root_node_list: OnceCell::new(),
};

pub fn initialize(available_range: Range<usize>) {
    com2_println!("NODE_LIST_LENGTH = {:#x?}", NODE_LIST_LENGTH);
    unsafe {
        ALLOCATOR.initialize(available_range);
    }
}

#[derive(Debug, Default)]
struct Allocator<'a> {
    root_node_list: OnceCell::<&'a mut NodeList>,
}

impl<'a> Allocator<'a> {
    pub fn initialize(&'a mut self, available_range: Range<usize>) {
        self.root_node_list
            .set(NodeList::new(available_range))
            .unwrap()
    }
}

unsafe impl GlobalAlloc for Allocator<'_> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        panic!("Global allocator is not implemented!");
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        panic!("Global allocator is not implemented!");
    }
}

#[derive(Debug)]
#[repr(align(4096))]
struct NodeList {
    nodes: [Node; NODE_LIST_LENGTH],
}

const NODE_LIST_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Node>();

impl NodeList {
    fn new<'a>(available_range: Range<usize>) -> &'a mut Self {
        let available_size: usize = available_range.len();
        let size: usize = available_size.next_power_of_two();
        let end: usize = available_range.end;
        let start: usize = end - size;
        let range: Range<usize> = start..end;
        let available_range: Range<usize> = available_range.start..available_range.end - memory::PAGE_SIZE;
        let node_list: usize = available_range.end;
        let node_list: *mut Self = node_list as *mut Self;
        let node_list: &mut Self = unsafe {
            &mut *node_list
        };
        *node_list = Self::default();
        node_list.nodes[0].initialize(range, available_range);
        com2_println!("node_list = {:#x?}", node_list);
        node_list
    }

    fn node_mut(&mut self, index: usize) -> &mut Node {
        &mut self.nodes[index]
    }
}

impl Default for NodeList {
    fn default() -> Self {
        const NODE: Node = Node::default();
        let nodes = [NODE; NODE_LIST_LENGTH];
        Self {
            nodes,
        }
    }
}

#[derive(Debug)]
struct Node {
    state: State,
    range: Range<usize>,
    available_range: Range<usize>,
    max_length: usize,
}

impl Node {
    fn add_higher_half_node(&mut self) -> &mut Self {
        match self.higher_half_node_index_in_list() {
            Some(index) => self
                .node_list_mut()
                .node_mut(index),
            None => panic!("Add a node list."),
        }
    }

    fn add_lower_half_node(&mut self) -> &mut Self {
        match self.lower_half_node_index_in_list() {
            Some(index) => self
                .node_list_mut()
                .node_mut(index),
            None => panic!("Add a node list."),
        }
    }

    const fn default() -> Self {
        Self {
            state: State::NotExist,
            range: 0..0,
            available_range: 0..0,
            max_length: 0,
        }
    }

    fn divide(&mut self) -> bool {
        let lower_half_range: Option<Range<usize>> = self.lower_half_range();
        let lower_half_available_range: Option<Range<usize>> = self.lower_half_available_range();
        let higher_half_range: Option<Range<usize>> = self.higher_half_range();
        let higher_half_available_range: Option<Range<usize>> = self.higher_half_available_range();
        com2_println!("divide");
        com2_println!("self.range = {:#x?}", self.range);
        com2_println!("self.available_range = {:#x?}", self.available_range);
        com2_println!("lower_half_range = {:#x?}", lower_half_range);
        com2_println!("lower_half_available_range = {:#x?}", lower_half_available_range);
        com2_println!("higher_half_range = {:#x?}", higher_half_range);
        com2_println!("higher_half_available_range = {:#x?}", higher_half_available_range);
        match (lower_half_available_range, higher_half_available_range) {
            (None, None) => false,
            (lower_half_available_range, higher_half_available_range) => {
                self.state.divide();
                if let (Some(lower_half_range), Some(lower_half_available_range)) = (lower_half_range, lower_half_available_range) {
                    self.add_lower_half_node()
                        .initialize(lower_half_range, lower_half_available_range);
                }
                if let (Some(higher_half_range), Some(higher_half_available_range)) = (higher_half_range, higher_half_available_range) {
                    self.add_higher_half_node()
                        .initialize(higher_half_range, higher_half_available_range);
                }
                true
            },
        }
    }

    fn divide_point(&self) -> usize {
        self.range.end / 2 + self.range.start / 2
    }

    fn higher_half_node_index_in_list(&self) -> Option<usize> {
        let index: usize = 2 * self.index_in_list() + 2;
        (index < NODE_LIST_LENGTH).then_some(index)
    }

    fn higher_half_available_range(&self) -> Option<Range<usize>> {
        let start: usize = [self.available_range.start, self.divide_point()]
            .into_iter()
            .max()
            .unwrap();
        let end: usize = self.available_range.end - self
            .higher_half_node_index_in_list()
            .map_or(memory::PAGE_SIZE, |_| 0);
        let range: Range<usize> = start..end;
        (!range.is_empty()).then_some(range)
    }

    fn higher_half_range(&self) -> Option<Range<usize>> {
        let start: usize = self.divide_point();
        let end: usize = self.range.end;
        let range: Range<usize> = start..end;
        (!range.is_empty()).then_some(range)
    }

    fn index_in_list(&self) -> usize {
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let offset: usize = address % memory::PAGE_SIZE;
        offset / mem::size_of::<Self>()
    }

    fn initialize(&mut self, range: Range<usize>, available_range: Range<usize>) {
        let state = State::Free;
        let max_length: usize = available_range.len();
        *self = Self {
            state,
            range,
            available_range,
            max_length,
        };
        if self.range.start != self.available_range.start {
            self.divide();
        }
    }

    fn lower_half_node_index_in_list(&self) -> Option<usize> {
        let index: usize = 2 * self.index_in_list() + 1;
        (index < NODE_LIST_LENGTH).then_some(index)
    }

    fn lower_half_available_range(&self) -> Option<Range<usize>> {
        let start: usize = self.available_range.start;
        let end: usize = [self.divide_point(), self.available_range.end]
            .into_iter()
            .min()
            .unwrap() - self
            .lower_half_node_index_in_list()
            .map_or(memory::PAGE_SIZE, |_| 0);
        let range: Range<usize> = start..end;
        (!range.is_empty()).then_some(range)
    }

    fn lower_half_range(&self) -> Option<Range<usize>> {
        let start: usize = self.range.start;
        let end: usize = self.divide_point();
        let range: Range<usize> = start..end;
        (!range.is_empty()).then_some(range)
    }

    fn node_list_mut(&mut self) -> &mut NodeList {
        let address: *mut Self = self as *mut Self;
        let address: usize = address as usize;
        let address: usize = (address / memory::PAGE_SIZE) * memory::PAGE_SIZE;
        let address: *mut NodeList = address as *mut NodeList;
        unsafe {
            &mut *address
        }
    }
}

#[derive(Debug)]
enum State {
    Allocated,
    Divided,
    Free,
    NotExist,
}

impl State {
    fn divide(&mut self) {
        assert!(matches!(self, Self::Free));
        *self = Self::Divided;
    }
}

