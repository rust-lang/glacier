extern "Rust" {
    fn foo();
}

pub extern fn bar() {
    unsafe {
        foo();
    }
}
