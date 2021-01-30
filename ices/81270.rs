#![feature(allocator_api)]
use std::alloc::{AllocError, Allocator, Global, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct S<A> {
    a: PhantomData<A>,
    b: [u8; 1],
}
unsafe impl<A> Allocator for S<A> {
    fn allocate(&self, _: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!();
    }
    unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {
        todo!();
    }
}

fn main() {
    let x: Box<u8, S<u8>> = Box::new_in(
        0,
        S {
            a: PhantomData,
            b: [0; 1],
        },
    );
}
