struct Foo<'a> { x: &'a () }

impl<'a> Foo<'a> {
    const X: usize = 0;

    fn foo(_: [u8; Self::X]) {}
}


fn main() {}
