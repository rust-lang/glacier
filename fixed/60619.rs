pub struct Foo<const LEN: usize> {
    buf: [u8; {LEN * 2}]
}

fn main() {}
