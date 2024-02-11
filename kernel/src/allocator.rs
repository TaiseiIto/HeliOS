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

pub fn initialize(heap_start: usize, heap_end: usize) {
    unsafe {
        ALLOCATOR.initialize(heap_start, heap_end);
    }
}

#[derive(Debug, Default)]
struct Allocator<'a> {
    root_node_list: OnceCell::<&'a mut NodeList>,
}

impl<'a> Allocator<'a> {
    pub fn initialize(&'a mut self, heap_start: usize, heap_end: usize) {
        self.root_node_list
            .set(NodeList::new(heap_start, heap_end))
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
    fn new<'a>(available_heap_start: usize, available_heap_end: usize) -> &'a mut Self {
        let available_heap_size: usize = available_heap_end - available_heap_start;
        let heap_size: usize = available_heap_size.next_power_of_two();
        let heap_end: usize = available_heap_end;
        let heap_start: usize = heap_end - heap_size;
        let available_heap_end: usize = heap_end - memory::PAGE_SIZE;
        let node_list: usize = available_heap_end;
        let node_list: *mut Self = node_list as *mut Self;
        let node_list: &mut Self = unsafe {
            &mut *node_list
        };
        *node_list = Self::default();
        node_list.nodes[0].initialize(heap_start, heap_end, available_heap_start, available_heap_end);
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
    start: usize,
    end: usize,
    available_start: usize,
    available_end: usize,
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
            start: 0,
            end: 0,
            available_start: 0,
            available_end: 0,
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
        self.end / 2 + self.start / 2
    }

    fn higher_half_node_index_in_list(&self) -> usize {
        2 * self.index_in_list() + 2
    }

    fn higher_half_range(&self) -> Range<usize> {
        self.divide_point()..self.end
    }

    fn index_in_list(&self) -> usize {
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let offset: usize = address % memory::PAGE_SIZE;
        offset / mem::size_of::<Self>()
    }

    fn initialize(&mut self, start: usize, end: usize, available_start: usize, available_end: usize) {
        let state = State::Free;
        let max_length: usize = available_end - available_start;
        *self = Self {
            state,
            start,
            end,
            available_start,
            available_end,
            max_length,
        };
        if start != available_start {
            self.divide();
        }
    }

    fn lower_half_node_index_in_list(&self) -> usize {
        2 * self.index_in_list() + 1
    }

    fn lower_half_range(&self) -> Range<usize> {
        self.start..self.divide_point()
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

