macro_rules! foo {
    ($e:expr) => { $e.foo() }
}

fn main() {
    let (a,) = 1i32.foo();
    foo!(a);
}
