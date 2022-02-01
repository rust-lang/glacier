trait Foo {
    type Bar;
}

trait Baz: Foo {
    const Bar: Self::Bar;
}

pub fn main() {}
