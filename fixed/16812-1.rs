enum Foo {
    Bar(u32, [u32]),
}

fn main() {
    // Either of these lines can cause the ICE
    let _x: &(u32, [u32]);
    let _y: &Foo;
}
