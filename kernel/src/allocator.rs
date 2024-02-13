//! # A memory allocator

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell::OnceCell,
        cmp,
        fmt,
        mem,
        ops::Range,
        slice,
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
        com2_println!("ALLOCATOR = {:#x?}", ALLOCATOR);
    }
}

#[derive(Debug, Default)]
struct Allocator<'a> {
    root_node_list: OnceCell::<&'a mut NodeList>,
}

impl<'a> Allocator<'a> {
    pub fn initialize(&'a mut self, available_range: Range<usize>) {
        let available_start: usize = available_range.start;
        let end: usize = available_range.end;
        let available_end: usize = end - memory::PAGE_SIZE;
        let size: usize = (end - available_start).next_power_of_two();
        let start: usize = end - size;
        let range: Range<usize> = start..end;
        let available_range: Range<usize> = available_start..available_end;
        self.root_node_list
            .set(NodeList::new(range, available_range))
            .unwrap()
    }
}

unsafe impl GlobalAlloc for Allocator<'_> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        panic!("Unimplemented!");
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        panic!("Unimplemented!");
    }
}

#[repr(align(4096))]
struct NodeList {
    nodes: [Node; NODE_LIST_LENGTH],
}

const NODE_LIST_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Node>();

impl NodeList {
    fn alloc(&mut self, layout: Layout) -> Option<*mut u8> {
        let align: usize = layout.align();
        let size: usize = layout.size();
        let size: usize = size.next_power_of_two();
        let size: usize = cmp::max(align, size);
        self.nodes[0].alloc(size)
    }

    fn dealloc(&mut self, address: usize, layout: Layout) {
        panic!("Unimplemented!")
    }

    fn new<'a>(range: Range<usize>, available_range: Range<usize>) -> &'a mut Self {
        let node_list: usize = available_range.end;
        let node_list: *mut Self = node_list as *mut Self;
        let node_list: &mut Self = unsafe {
            &mut *node_list
        };
        *node_list = Self::default();
        node_list.nodes[0].initialize(range, available_range);
        node_list
    }

    fn mut_node(&mut self, index: usize) -> &mut Node {
        &mut self.nodes[index]
    }

    fn node(&self, index: usize) -> &Node {
        &self.nodes[index]
    }
}

impl fmt::Debug for NodeList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NodeList")
            .field("root", &self.nodes[0])
            .finish()
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

struct Node {
    state: State,
    range: Range<usize>,
    available_range: Range<usize>,
    max_length: usize,
}

impl Node {
    fn add_higher_half_node(&mut self) -> Option<&mut Self> {
        if let Some(higher_half_node_index_in_list) = self.higher_half_node_index_in_list() {
            Some(self.mut_node_list()
                .mut_node(higher_half_node_index_in_list))
        } else if let (Some(higher_half_range), Some(higher_half_available_range)) = (self.higher_half_range(), self.higher_half_available_range()) {
            Some(NodeList::new(higher_half_range, higher_half_available_range).mut_node(0))
        } else {
            None
        }
    }

    fn add_lower_half_node(&mut self) -> Option<&mut Self> {
        if let Some(lower_half_node_index_in_list) = self.lower_half_node_index_in_list() {
            Some(self.mut_node_list()
                .mut_node(lower_half_node_index_in_list))
        } else if let (Some(lower_half_range), Some(lower_half_available_range)) = (self.lower_half_range(), self.lower_half_available_range()) {
            Some(NodeList::new(lower_half_range, lower_half_available_range).mut_node(0))
        } else {
            None
        }
    }

    fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        let allocated: Option<*mut u8> = match self.state {
            State::Allocated | State::NotExist => None,
            State::Divided => if self
                .get_lower_half_node()
                .map(|lower_half_node| lower_half_node.max_length)
                .filter(|lower_half_max_length| size <= *lower_half_max_length)
                .is_some() {
                self.get_mut_lower_half_node()
                    .and_then(|lower_half_node| lower_half_node.alloc(size))
            } else if self
                .get_higher_half_node()
                .map(|higher_half_node| higher_half_node.max_length)
                .filter(|higher_half_max_length| size <= *higher_half_max_length)
                .is_some() {
                self.get_mut_higher_half_node()
                    .and_then(|higher_half_node| higher_half_node.alloc(size))
            } else {
                None
            },
            State::Free => {
                self.divide();
                if matches!(self.state, State::Divided) && size <= self.max_length {
                    self.alloc(size)
                } else {
                    if matches!(self.state, State::Divided) {
                        self.merge();
                    }
                    self.state = State::Allocated;
                    self.get_mut()
                }
            },
        };
        self.update_max_length();
        allocated
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
        let lower_half_range: Option<Range<usize>> = self.lower_half_range();
        let lower_half_available_range: Option<Range<usize>> = self.lower_half_available_range();
        let higher_half_range: Option<Range<usize>> = self.higher_half_range();
        let higher_half_available_range: Option<Range<usize>> = self.higher_half_available_range();
        if let (Some(lower_half_range), Some(lower_half_available_range), Some(lower_half_node)) = (lower_half_range, lower_half_available_range, self.add_lower_half_node()) {
            lower_half_node.initialize(lower_half_range, lower_half_available_range);
            self.state = State::Divided
        }
        if let (Some(higher_half_range), Some(higher_half_available_range), Some(higher_half_node)) = (higher_half_range, higher_half_available_range, self.add_higher_half_node()) {
            higher_half_node.initialize(higher_half_range, higher_half_available_range);
            self.state = State::Divided
        }
        self.update_max_length();
    }

    fn divide_point(&self) -> usize {
        self.range.end / 2 + self.range.start / 2
    }

    fn get_higher_half_node(&self) -> Option<&Self> {
        if matches!(self.state, State::Divided) {
            if let Some(higher_half_node_index_in_list) = self.higher_half_node_index_in_list() {
                let higher_half_node: &Self = self
                    .node_list()
                    .node(higher_half_node_index_in_list);
                (higher_half_node.state != State::NotExist).then_some(higher_half_node)
            } else if let Some(higher_half_available_range) = self.higher_half_available_range() {
                let node_list: usize = higher_half_available_range.end;
                let node_list: *const NodeList = node_list as *const NodeList;
                let node_list: &NodeList = unsafe {
                    &*node_list
                };
                let higher_half_node: &Self = &node_list.nodes[0];
                (higher_half_node.state != State::NotExist).then_some(higher_half_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut_higher_half_node(&mut self) -> Option<&mut Self> {
        if matches!(self.state, State::Divided) {
            if let Some(higher_half_node_index_in_list) = self.higher_half_node_index_in_list() {
                let higher_half_node: &mut Self = self
                    .mut_node_list()
                    .mut_node(higher_half_node_index_in_list);
                (higher_half_node.state != State::NotExist).then_some(higher_half_node)
            } else if let Some(higher_half_available_range) = self.higher_half_available_range() {
                let node_list: usize = higher_half_available_range.end;
                let node_list: *mut NodeList = node_list as *mut NodeList;
                let node_list: &mut NodeList = unsafe {
                    &mut *node_list
                };
                let higher_half_node: &mut Self = &mut node_list.nodes[0];
                (higher_half_node.state != State::NotExist).then_some(higher_half_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_lower_half_node(&self) -> Option<&Self> {
        if matches!(self.state, State::Divided) {
            if let Some(lower_half_node_index_in_list) = self.lower_half_node_index_in_list() {
                let lower_half_node: &Self = self
                    .node_list()
                    .node(lower_half_node_index_in_list);
                (lower_half_node.state != State::NotExist).then_some(lower_half_node)
            } else if let Some(lower_half_available_range) = self.lower_half_available_range() {
                let node_list: usize = lower_half_available_range.end;
                let node_list: *const NodeList = node_list as *const NodeList;
                let node_list: &NodeList = unsafe {
                    &*node_list
                };
                let lower_half_node: &Self = &node_list.nodes[0];
                (lower_half_node.state != State::NotExist).then_some(lower_half_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut_lower_half_node(&mut self) -> Option<&mut Self> {
        if matches!(self.state, State::Divided) {
            if let Some(lower_half_node_index_in_list) = self.lower_half_node_index_in_list() {
                let lower_half_node: &mut Self = self
                    .mut_node_list()
                    .mut_node(lower_half_node_index_in_list);
                (lower_half_node.state != State::NotExist).then_some(lower_half_node)
            } else if let Some(lower_half_available_range) = self.lower_half_available_range() {
                let node_list: usize = lower_half_available_range.end;
                let node_list: *mut NodeList = node_list as *mut NodeList;
                let node_list: &mut NodeList = unsafe {
                    &mut *node_list
                };
                let lower_half_node: &mut Self = &mut node_list.nodes[0];
                (lower_half_node.state != State::NotExist).then_some(lower_half_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut(&mut self) -> Option<*mut u8> {
        matches!(self.state, State::Free).then(|| self.available_range.start as *mut u8)
    }

    fn higher_half_node_index_in_list(&self) -> Option<usize> {
        let index: usize = 2 * self.index_in_list() + 2;
        (index < NODE_LIST_LENGTH).then_some(index)
    }

    fn higher_half_available_range(&self) -> Option<Range<usize>> {
        let start: usize = cmp::max(self.available_range.start, self.divide_point());
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
        assert!(!range.is_empty());
        assert!(!available_range.is_empty());
        assert!(range.start <= available_range.start);
        assert!(available_range.end <= range.end);
        assert!(range.len().is_power_of_two());
        assert_eq!((range.start / range.len()) * range.len(), range.start);
        assert_eq!((range.end / range.len()) * range.len(), range.end);
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
        let end: usize = cmp::min(self.divide_point(), self.available_range.end) - self
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

    fn merge(&mut self) {
        if matches!(self.state, State::Divided) {
            self.state = State::Free;
            self.max_length = self.available_range.len();
        }
    }

    fn mut_node_list(&mut self) -> &mut NodeList {
        let address: *mut Self = self as *mut Self;
        let address: usize = address as usize;
        let address: usize = (address / memory::PAGE_SIZE) * memory::PAGE_SIZE;
        let address: *mut NodeList = address as *mut NodeList;
        unsafe {
            &mut *address
        }
    }

    fn node_list(&self) -> &NodeList {
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let address: usize = (address / memory::PAGE_SIZE) * memory::PAGE_SIZE;
        let address: *const NodeList = address as *const NodeList;
        unsafe {
            &*address
        }
    }

    fn update_max_length(&mut self) {
        if self.state == State::Divided {
            let lower_half_max_length: Option<usize> = self
                .get_lower_half_node()
                .map(|lower_half_node| lower_half_node.max_length);
            let higher_half_max_length: Option<usize> = self
                .get_higher_half_node()
                .map(|higher_half_node| higher_half_node.max_length);
            self.max_length = [lower_half_max_length, higher_half_max_length]
                .into_iter()
                .flatten()
                .max()
                .unwrap();
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Node")
            .field("state", &self.state)
            .field("range", &self.range)
            .field("available_range", &self.available_range)
            .field("max_length", &self.max_length)
            .field("lower_half", &self.get_lower_half_node())
            .field("higher_half", &self.get_higher_half_node())
            .finish()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    Allocated,
    Divided,
    Free,
    NotExist,
}

