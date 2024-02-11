//! # A memory allocator

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell::OnceCell,
        mem,
    },
    crate::memory,
};

#[global_allocator]
static mut ALLOCATOR: Allocator<'static> = Allocator {
    root_node_list: OnceCell::new(),
};

#[derive(Debug, Default)]
struct Allocator<'a> {
    root_node_list: OnceCell::<&'a mut NodeList>,
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

impl Default for NodeList {
    fn default() -> Self {
        const none: Option<Node> = None;
        let nodes: [Option<Node>; NODE_LIST_LENGTH] = [none; NODE_LIST_LENGTH];
        Self {
            nodes,
        }
    }
}

const NODE_LIST_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Option<Node>>();

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

