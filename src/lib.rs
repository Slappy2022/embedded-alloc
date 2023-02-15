#![doc = include_str!("../README.md")]
#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use linked_list_allocator::Heap as LLHeap;

pub struct Heap {
    heap: RefCell<LLHeap>,
}

impl Heap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](Self::init) method before using the allocator.
    pub const fn empty() -> Heap {
        Heap {
            heap: RefCell::new(LLHeap::empty()),
        }
    }

    /// Initializes the heap
    ///
    /// This function must be called BEFORE you run any code that makes use of the
    /// allocator.
    ///
    /// `start_addr` is the address where the heap will be located.
    ///
    /// `size` is the size of the heap in bytes.
    ///
    /// Note that:
    ///
    /// - The heap grows "upwards", towards larger addresses. Thus `start_addr` will
    ///   be the smallest address used.
    ///
    /// - The largest address used is `start_addr + size - 1`, so if `start_addr` is
    ///   `0x1000` and `size` is `0x30000` then the allocator won't use memory at
    ///   addresses `0x31000` and larger.
    ///
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        self.heap.borrow_mut().init(start_addr as *mut u8, size)
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        self.heap.borrow_mut().used()
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        self.heap.borrow_mut().free()
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.heap
            .borrow_mut()
            .allocate_first_fit(layout)
            .ok()
            .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.heap
            .borrow_mut()
            .deallocate(NonNull::new_unchecked(ptr), layout)
    }
}
