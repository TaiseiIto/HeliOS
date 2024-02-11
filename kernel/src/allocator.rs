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

impl Allocator<'_> {
    pub fn initialize(&mut self, available_heap_start: usize, available_heap_end: usize) {
        let available_heap_size: usize = available_heap_end - available_heap_start;
        let heap_size: usize = available_heap_size.next_power_of_two();
        let heap_end: usize = available_heap_end;
        let heap_start: usize = heap_end - heap_size;
        com2_println!("available_heap_start = {:#x?}", available_heap_start);
        com2_println!("available_heap_size = {:#x?}", available_heap_size);
        com2_println!("available_heap_end = {:#x?}", available_heap_end);
        com2_println!("heap_start = {:#x?}", heap_start);
        com2_println!("heap_size = {:#x?}", heap_size);
        com2_println!("heap_end = {:#x?}", heap_end);
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
    nodes: [Option<Node>; NODE_LIST_LENGTH],
}

const NODE_LIST_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Option<Node>>();

impl Default for NodeList {
    fn default() -> Self {
        const none: Option<Node> = None;
        let nodes: [Option<Node>; NODE_LIST_LENGTH] = [none; NODE_LIST_LENGTH];
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

#[derive(Debug)]
enum State {
    Allocated,
    Divided,
    Free,
}

