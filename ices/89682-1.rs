#![feature(allocator_api)]

use core::alloc::{AllocError, Allocator, Layout};
use core::ptr::NonNull;

struct AwfulAllocator<const N: usize>([u64; N]);

unsafe impl<const N: usize> Allocator for AwfulAllocator<N> {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }
    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        todo!()
    }
}

fn main() {
    let f = AwfulAllocator([0; 128]);
    let _x = Box::<i32, AwfulAllocator<128>>::new_in(43, f);
}
