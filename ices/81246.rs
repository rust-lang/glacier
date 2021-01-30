#![feature(allocator_api)]

fn ice() -> Box<(), &'static std::alloc::Global> {
    todo!()
}

fn main() {
    ice();
}
