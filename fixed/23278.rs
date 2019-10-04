extern "Rust" {
    fn foo();
}

fn main() {
    unsafe { foo() };
}

