extern { pub static FOO: extern "rust-intrinsic" fn(); }
fn main() { FOO() }
