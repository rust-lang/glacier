pub trait Bar {
    type Type;
}
pub struct Foo<'a>(&'a ());
impl<'a> Bar for Foo<'a> {
    type Type = ();
}

pub fn func<'a>(_: <Foo<'a> as Bar>::Type) {}
pub fn assert_is_func<A>(_: fn(A)) {}

pub fn test()
where
    for<'a> <Foo<'a> as Bar>::Type: Sized,
{
    assert_is_func(func);
}

fn main() {}
