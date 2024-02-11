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
        com2_println!("available_range = {:#x?}", available_range);
        let available_size: usize = available_range.len();
        com2_println!("available_size = {:#x?}", available_size);
        let size: usize = available_size.next_power_of_two();
        com2_println!("size = {:#x?}", size);
        let end: usize = available_range.end;
        com2_println!("end = {:#x?}", end);
        let start: usize = end - size;
        com2_println!("start = {:#x?}", start);
        let range: Range<usize> = start..end;
        let available_range: Range<usize> = available_range.start..available_range.end - memory::PAGE_SIZE;
        com2_println!("range = {:#x?}", range);
        let node_list: usize = available_range.end;
        com2_println!("node_list = {:#x?}", node_list);
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
    fn add_lower_half_node(&mut self) -> &mut Self {
        let lower_half_node_index_in_list: usize = self.lower_half_node_index_in_list();
        if lower_half_node_index_in_list < NODE_LIST_LENGTH {
            self
                .node_list_mut()
                .node_mut(lower_half_node_index_in_list)
        } else {
            panic!("Add a node list.")
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

    fn divide(&mut self) {
        self.state.divide();
        let lower_half_node: &mut Self = self.add_lower_half_node();
        let higher_half_range: Range<usize> = self.higher_half_range();
        let lower_half_range: Range<usize> = self.lower_half_range();
        com2_println!("higher_half_range = {:#x?}", higher_half_range);
        com2_println!("lower_half_range = {:#x?}", lower_half_range);
    }

    fn divide_point(&self) -> usize {
        self.range.end / 2 + self.range.start / 2
    }

    fn higher_half_node_index_in_list(&self) -> usize {
        2 * self.index_in_list() + 2
    }

    fn higher_half_range(&self) -> Range<usize> {
        self.divide_point()..self.range.end
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

    fn lower_half_node_index_in_list(&self) -> usize {
        2 * self.index_in_list() + 1
    }

    fn lower_half_range(&self) -> Range<usize> {
        self.range.start..self.divide_point()
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

