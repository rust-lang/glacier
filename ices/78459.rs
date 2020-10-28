#![feature(allocator_api)]

fn main() {
    Box::new_in(&[0, 1], &std::alloc::Global);
}
