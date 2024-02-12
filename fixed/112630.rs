enum Foo {
    Bar(B),
}

fn main() {
    let _ = [0u32; Foo::Bar as usize];
}
