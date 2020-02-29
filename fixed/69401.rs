struct Inv<'a> {
    x: &'a u8
}
pub trait Foo {
    fn no_bound(b: Inv<(b)>);
}

fn main() {}
