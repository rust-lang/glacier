#![feature(allocator_api)]

use std::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
use std::ptr::NonNull;

struct MyAllocator {}

impl MyAllocator {
    fn new() -> Self {
        Self {}
    }
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        std::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[derive(Clone, Debug)]
struct RefAlloc<'a, T: GlobalAlloc>(&'a T);

impl<'a, T: GlobalAlloc> RefAlloc<'a, T> {
    fn new(inner: &'a T) -> Self {
        Self(inner)
    }
}

unsafe impl<'a, T: GlobalAlloc> Allocator for RefAlloc<'a, T> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { self.0.alloc(layout) };

        if ptr.is_null() {
            return Err(AllocError);
        }

        unsafe {
            Ok(NonNull::new_unchecked(std::slice::from_raw_parts_mut(
                ptr,
                layout.size(),
            )))
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.0.dealloc(ptr.as_ptr(), layout)
    }
}

fn main() {
    let my_alloc: MyAllocator = MyAllocator::new();

    // Constructing a box using the RefAlloc causes the compiler to crash
    let a_box = Box::new_in(0, RefAlloc::new(&my_alloc));

    println!("Hello, world! {:?}", a_box);
}
