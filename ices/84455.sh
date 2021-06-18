#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "abc"
version = "0.0.1"
edition = "2018"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
EOF

mkdir -p src

cat > src/lib.rs <<'EOF'
// ice_test
#![no_std]

#![feature(const_fn)]

use core::alloc::{GlobalAlloc, Layout};

pub static DEFAULT_ALLOCATOR: Allocator = Allocator::new(&DEFAULT_HEAP);
static DEFAULT_HEAP: GeneralAllocator = GeneralAllocator::new();

#[derive(Copy, Clone)]
pub struct Allocator {
    allocator: &'static (dyn GlobalAlloc + 'static),
}

unsafe impl Sync for Allocator {}

impl Allocator {
    const fn new(allocator: &'static dyn GlobalAlloc) -> Self {
        Self { allocator }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocator.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.allocator.dealloc(ptr, layout)
    }
}

pub struct GeneralAllocator;

unsafe impl Sync for GeneralAllocator {}

impl GeneralAllocator {
    pub const fn new() -> Self {
        Self {}
    }
}

unsafe impl GlobalAlloc for GeneralAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}
EOF

cat > src/test.rs <<'EOF'
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

use ice_test::{Allocator, DEFAULT_ALLOCATOR};


extern crate alloc;

#[global_allocator]
pub static GLOBAL_ALLOCATOR: Allocator = DEFAULT_ALLOCATOR;

#[no_mangle]
fn main() {

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!(
        "Error allocating  {} bytes of memory with alignment {}",
        layout.size(),
        layout.align()
    );
}
EOF

cargo test
