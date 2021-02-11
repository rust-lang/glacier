use std::cell::UnsafeCell;

extern "C" {
    pub fn foo(_: Option<UnsafeCell<&i32>>);
}

fn main() {}
