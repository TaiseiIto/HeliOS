//! # A memory allocator

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell::OnceCell,
        mem,
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
        com2_println!("available_heap_start = {:#x?}", available_heap_start);
        com2_println!("available_heap_size = {:#x?}", available_heap_size);
        com2_println!("available_heap_end = {:#x?}", available_heap_end);
        com2_println!("heap_start = {:#x?}", heap_start);
        com2_println!("heap_size = {:#x?}", heap_size);
        com2_println!("heap_end = {:#x?}", heap_end);
        let node_list: usize = available_heap_end;
        let node_list: *mut Self = node_list as *mut Self;
        let node_list: &mut Self = unsafe {
            &mut *node_list
        };
        *node_list = Self::default();
        com2_println!("node_list = {:#x?}", node_list);
        node_list
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
    index_in_list: usize,
}

impl Node {
    const fn default() -> Self {
        Self {
            state: State::NotExist,
            start: 0,
            end: 0,
            available_start: 0,
            available_end: 0,
            max_length: 0,
            index_in_list: 0,
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

