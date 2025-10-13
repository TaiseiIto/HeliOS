pub mod page;
pub mod paging;
pub mod segment;
pub mod stack;

pub use {
    page::{ContinuousPages, Page},
    paging::Paging,
    stack::Stack,
};

use {
    crate::{efi, task},
    alloc::alloc::Layout,
    core::{alloc::GlobalAlloc, cell::UnsafeCell, cmp, fmt, mem::size_of, ops::Range},
};

pub const KIB: usize = 1 << 10;

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator {
    root_node_list: UnsafeCell::new(0 as *mut NodeList),
};

pub fn initialize(paging: &mut Paging, memory_map: &efi::memory::Map, heap_start: usize) -> usize {
    let heap_end: usize = memory_map
        .iter()
        .filter(|memory_descriptor| memory_descriptor.is_available())
        .flat_map(|memory_descriptor| memory_descriptor.physical_range().step_by(page::SIZE))
        .enumerate()
        .map(|(index, paddr)| {
            let vaddr: usize = heap_start + index * page::SIZE;
            let present: bool = true;
            let writable: bool = true;
            let executable: bool = false;
            paging.set_page(vaddr, paddr, present, writable, executable);
            vaddr + page::SIZE
        })
        .max()
        .unwrap();
    unsafe {
        ALLOCATOR.initialize(heap_start..heap_end);
    }
    heap_end - heap_start
}

struct Allocator {
    root_node_list: UnsafeCell<*mut NodeList>,
}

impl Allocator {
    pub fn get_root_node_list(&self) -> &NodeList {
        let root_node_list: *mut *mut NodeList = self.root_node_list.get();
        let root_node_list: *mut NodeList = unsafe { *root_node_list };
        let root_node_list: *const NodeList = root_node_list as *const NodeList;
        unsafe { &*root_node_list }
    }

    pub fn initialize(&mut self, available_range: Range<usize>) {
        let available_start: usize = available_range.start;
        let available_end: usize = available_range.end - page::SIZE;
        let available_range: Range<usize> = available_start..available_end;
        let available_size: usize = available_range.len();
        let size: usize = available_size.next_power_of_two();
        let start: usize = available_start;
        let end: usize = start + size;
        let range: Range<usize> = start..end;
        let node_list: &mut NodeList = NodeList::new(range, available_range);
        let node_list: *mut NodeList = node_list as *mut NodeList;
        *unsafe { &mut *self.root_node_list.get() } = node_list;
    }
}

impl fmt::Debug for Allocator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Allocator")
            .field("root_node_list", self.get_root_node_list())
            .finish()
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(current_task) = task::Controller::get_current_mut() {
            current_task.cli();
        }
        let root_node_list: *mut *mut NodeList = self.root_node_list.get();
        let root_node_list: *mut NodeList = *root_node_list;
        let root_node_list: &mut NodeList = &mut *root_node_list;
        let allocated: *mut u8 = root_node_list.alloc(layout).unwrap();
        if let Some(current_task) = task::Controller::get_current_mut() {
            current_task.sti()
        };
        allocated
    }

    unsafe fn dealloc(&self, address: *mut u8, _: Layout) {
        if let Some(current_task) = task::Controller::get_current_mut() {
            current_task.cli();
        }
        let root_node_list: *mut *mut NodeList = self.root_node_list.get();
        let root_node_list: *mut NodeList = *root_node_list;
        let root_node_list: &mut NodeList = &mut *root_node_list;
        root_node_list.dealloc(address);
        if let Some(current_task) = task::Controller::get_current_mut() {
            current_task.sti();
        }
    }
}

#[repr(align(4096))]
struct NodeList {
    nodes: [Node; NODE_LIST_LENGTH],
}

const NODE_LIST_LENGTH: usize = page::SIZE / size_of::<Node>();

impl NodeList {
    fn alloc(&mut self, layout: Layout) -> Option<*mut u8> {
        let align: usize = layout.align();
        let size: usize = layout.size();
        let size: usize = size.next_power_of_two();
        let size: usize = cmp::max(align, size);
        self.nodes[0].alloc(size)
    }

    fn dealloc(&mut self, address: *mut u8) {
        self.nodes[0].dealloc(address);
    }

    fn new<'a>(range: Range<usize>, available_range: Range<usize>) -> &'a mut Self {
        let node_list: usize = available_range.end;
        let node_list: *mut Self = node_list as *mut Self;
        let node_list: &mut Self = unsafe { &mut *node_list };
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
        Self { nodes }
    }
}

struct Node {
    state: State,
    start: usize,
    log_size: u8,
    unavailable_tail_size: usize,
    max_size: usize,
}

impl Node {
    fn add_higher_half_node(&mut self) -> Option<&mut Self> {
        if let Some(higher_half_node_index_in_list) = self.higher_half_node_index_in_list() {
            Some(
                self.mut_node_list()
                    .mut_node(higher_half_node_index_in_list),
            )
        } else if let (Some(higher_half_range), Some(higher_half_available_range)) =
            (self.higher_half_range(), self.higher_half_available_range())
        {
            Some(NodeList::new(higher_half_range, higher_half_available_range).mut_node(0))
        } else {
            None
        }
    }

    fn add_lower_half_node(&mut self) -> Option<&mut Self> {
        if let Some(lower_half_node_index_in_list) = self.lower_half_node_index_in_list() {
            Some(self.mut_node_list().mut_node(lower_half_node_index_in_list))
        } else if let (Some(lower_half_range), Some(lower_half_available_range)) =
            (self.lower_half_range(), self.lower_half_available_range())
        {
            Some(NodeList::new(lower_half_range, lower_half_available_range).mut_node(0))
        } else {
            None
        }
    }

    fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        let allocated: Option<*mut u8> = match self.state {
            State::Allocated | State::Invalid => None,
            State::Divided => {
                if let Some(higher_half_node) =
                    self.get_mut_higher_half_node().filter(|higher_half_node| {
                        matches!(higher_half_node.state, State::Divided | State::Free)
                            && size <= higher_half_node.max_size
                    })
                {
                    higher_half_node.alloc(size)
                } else if let Some(lower_half_node) =
                    self.get_mut_lower_half_node().filter(|lower_half_node| {
                        matches!(lower_half_node.state, State::Divided | State::Free)
                            && size <= lower_half_node.max_size
                    })
                {
                    lower_half_node.alloc(size)
                } else {
                    None
                }
            }
            State::Free => {
                self.divide();
                if matches!(self.state, State::Divided) && size <= self.max_size {
                    self.alloc(size)
                } else {
                    if matches!(self.state, State::Divided) {
                        self.merge();
                    }
                    self.state = State::Allocated;
                    self.get_mut()
                }
            }
        };
        self.update_max_size();
        allocated
    }

    fn available_range(&self) -> Range<usize> {
        let Range::<usize> {
            start,
            end,
        } = self.range();
        let available_end: usize = end - self.unavailable_tail_size;
        start..available_end
    }

    fn dealloc(&mut self, address: *mut u8) {
        match self.state {
            State::Allocated => {
                assert_eq!(self.get_mut(), Some(address));
                self.state = State::Free;
            }
            State::Divided => {
                if let Some(higher_half_node) =
                    self.get_mut_higher_half_node().filter(|higher_half_node| {
                        higher_half_node
                            .available_range()
                            .contains(&(address as usize))
                    })
                {
                    higher_half_node.dealloc(address);
                } else if let Some(lower_half_node) =
                    self.get_mut_lower_half_node().filter(|lower_half_node| {
                        lower_half_node
                            .available_range()
                            .contains(&(address as usize))
                    })
                {
                    lower_half_node.dealloc(address);
                } else {
                    panic!("Can't deallocate memory!");
                }
                if self.get_lower_half_node().map_or(false, |lower_half_node| {
                    lower_half_node.state == State::Free
                }) && self
                    .get_higher_half_node()
                    .map_or(false, |higher_half_node| {
                        higher_half_node.state == State::Free
                    })
                {
                    self.merge();
                }
            }
            State::Free => panic!("Double free!"),
            State::Invalid => panic!("Can't deallocate memory!"),
        }
        self.update_max_size();
    }

    const fn default() -> Self {
        Self {
            state: State::Invalid,
            start: 0,
            log_size: 0,
            unavailable_tail_size: 0,
            max_size: 0,
        }
    }

    fn divide(&mut self) {
        let lower_half_range: Option<Range<usize>> = self.lower_half_range();
        let lower_half_available_range: Option<Range<usize>> = self.lower_half_available_range();
        let higher_half_range: Option<Range<usize>> = self.higher_half_range();
        let higher_half_available_range: Option<Range<usize>> = self.higher_half_available_range();
        if let (Some(lower_half_range), Some(lower_half_available_range), Some(lower_half_node)) = (
            lower_half_range,
            lower_half_available_range,
            self.add_lower_half_node(),
        ) {
            lower_half_node.initialize(lower_half_range, lower_half_available_range);
            self.state = State::Divided
        }
        if let (
            Some(higher_half_range),
            Some(higher_half_available_range),
            Some(higher_half_node),
        ) = (
            higher_half_range,
            higher_half_available_range,
            self.add_higher_half_node(),
        ) {
            higher_half_node.initialize(higher_half_range, higher_half_available_range);
            self.state = State::Divided
        }
        self.update_max_size();
    }

    fn divide_point(&self) -> usize {
        let Range::<usize> {
            start,
            end,
        } = self.range();
        ((end as u128 + start as u128) / 2) as usize
    }

    fn get_higher_half_node(&self) -> Option<&Self> {
        if matches!(self.state, State::Divided) {
            if let Some(higher_half_node_index_in_list) = self.higher_half_node_index_in_list() {
                let higher_half_node: &Self = self.node_list().node(higher_half_node_index_in_list);
                (higher_half_node.state != State::Invalid).then_some(higher_half_node)
            } else if let Some(higher_half_available_range) = self.higher_half_available_range() {
                let node_list: usize = higher_half_available_range.end;
                let node_list: *const NodeList = node_list as *const NodeList;
                let node_list: &NodeList = unsafe { &*node_list };
                let higher_half_node: &Self = &node_list.nodes[0];
                (higher_half_node.state != State::Invalid).then_some(higher_half_node)
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
                (higher_half_node.state != State::Invalid).then_some(higher_half_node)
            } else if let Some(higher_half_available_range) = self.higher_half_available_range() {
                let node_list: usize = higher_half_available_range.end;
                let node_list: *mut NodeList = node_list as *mut NodeList;
                let node_list: &mut NodeList = unsafe { &mut *node_list };
                let higher_half_node: &mut Self = &mut node_list.nodes[0];
                (higher_half_node.state != State::Invalid).then_some(higher_half_node)
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
                let lower_half_node: &Self = self.node_list().node(lower_half_node_index_in_list);
                (lower_half_node.state != State::Invalid).then_some(lower_half_node)
            } else if let Some(lower_half_available_range) = self.lower_half_available_range() {
                let node_list: usize = lower_half_available_range.end;
                let node_list: *const NodeList = node_list as *const NodeList;
                let node_list: &NodeList = unsafe { &*node_list };
                let lower_half_node: &Self = &node_list.nodes[0];
                (lower_half_node.state != State::Invalid).then_some(lower_half_node)
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
                let lower_half_node: &mut Self =
                    self.mut_node_list().mut_node(lower_half_node_index_in_list);
                (lower_half_node.state != State::Invalid).then_some(lower_half_node)
            } else if let Some(lower_half_available_range) = self.lower_half_available_range() {
                let node_list: usize = lower_half_available_range.end;
                let node_list: *mut NodeList = node_list as *mut NodeList;
                let node_list: &mut NodeList = unsafe { &mut *node_list };
                let lower_half_node: &mut Self = &mut node_list.nodes[0];
                (lower_half_node.state != State::Invalid).then_some(lower_half_node)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut(&mut self) -> Option<*mut u8> {
        matches!(self.state, State::Allocated).then(|| self.available_range().start as *mut u8)
    }

    fn higher_half_node_index_in_list(&self) -> Option<usize> {
        let index: usize = 2 * self.index_in_list() + 2;
        Some(index).filter(|index| *index < NODE_LIST_LENGTH)
    }

    fn higher_half_available_range(&self) -> Option<Range<usize>> {
        let available_range: Range<usize> = self.available_range();
        let start: usize = cmp::max(available_range.start, self.divide_point());
        let end: usize = available_range.end
            - self
                .higher_half_node_index_in_list()
                .map_or(page::SIZE, |_| 0);
        Some(start..end).filter(|range| !range.is_empty())
    }

    fn higher_half_range(&self) -> Option<Range<usize>> {
        let start: usize = self.divide_point();
        let end: usize = self.range().end;
        Some(start..end).filter(|range| !range.is_empty())
    }

    fn index_in_list(&self) -> usize {
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let offset: usize = address % page::SIZE;
        offset / size_of::<Self>()
    }

    fn initialize(&mut self, range: Range<usize>, available_range: Range<usize>) {
        let size: usize = range.len();
        assert!(!range.is_empty());
        assert!(!available_range.is_empty());
        assert!(range.start == available_range.start);
        assert!(available_range.end <= range.end);
        assert!(size.is_power_of_two());
        assert_eq!((range.start / size) * size, range.start);
        assert_eq!((range.end / size) * size, range.end);
        let state = State::Free;
        let start: usize = range.start;
        let log_size: u8 = size.ilog2() as u8;
        let unavailable_tail_size: usize = range.end - available_range.end;
        let max_size: usize = available_range.len();
        *self = Self {
            state,
            start,
            log_size,
            unavailable_tail_size,
            max_size,
        };
    }

    fn lower_half_node_index_in_list(&self) -> Option<usize> {
        let index: usize = 2 * self.index_in_list() + 1;
        Some(index).filter(|index| *index < NODE_LIST_LENGTH)
    }

    fn lower_half_available_range(&self) -> Option<Range<usize>> {
        let available_range: Range<usize> = self.available_range();
        let start: usize = available_range.start;
        let end: usize = cmp::min(self.divide_point(), available_range.end)
            - self
                .lower_half_node_index_in_list()
                .map_or(page::SIZE, |_| 0);
        Some(start..end).filter(|range| !range.is_empty())
    }

    fn lower_half_range(&self) -> Option<Range<usize>> {
        let start: usize = self.range().start;
        let end: usize = self.divide_point();
        Some(start..end).filter(|range| !range.is_empty())
    }

    fn merge(&mut self) {
        if matches!(self.state, State::Divided) {
            self.state = State::Free;
            self.max_size = self.available_range().len();
        }
    }

    fn mut_node_list(&mut self) -> &mut NodeList {
        let address: *mut Self = self as *mut Self;
        let address: usize = address as usize;
        let address: usize = (address / page::SIZE) * page::SIZE;
        let address: *mut NodeList = address as *mut NodeList;
        unsafe { &mut *address }
    }

    fn node_list(&self) -> &NodeList {
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let address: usize = (address / page::SIZE) * page::SIZE;
        let address: *const NodeList = address as *const NodeList;
        unsafe { &*address }
    }

    fn range(&self) -> Range<usize> {
        let start: usize = self.start;
        let size: usize = 1 << self.log_size;
        let end: usize = start + size;
        start..end
    }

    fn update_max_size(&mut self) {
        if self.state == State::Divided {
            let lower_half_max_size: Option<usize> = self
                .get_lower_half_node()
                .filter(|lower_half_node| {
                    matches!(lower_half_node.state, State::Divided | State::Free)
                })
                .map(|lower_half_node| lower_half_node.max_size);
            let higher_half_max_size: Option<usize> = self
                .get_higher_half_node()
                .filter(|higher_half_node| {
                    matches!(higher_half_node.state, State::Divided | State::Free)
                })
                .map(|higher_half_node| higher_half_node.max_size);
            self.max_size = [lower_half_max_size, higher_half_max_size]
                .into_iter()
                .flatten()
                .max()
                .unwrap_or(0);
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Node")
            .field("state", &self.state)
            .field("range", &self.range())
            .field("available_range", &self.available_range())
            .field("max_size", &self.max_size)
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
    Invalid,
}
