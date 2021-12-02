use std::borrow::Cow;

pub enum Recursive<'a>
where
    Recursive<'a>: ToOwned<Owned = Box<Recursive<'a>>>,
{
    Variant(MyCow<'a, Recursive<'a>>),
}

pub struct Wrapper<T>(T);

pub struct MyCow<'a, T: ToOwned<Owned = Box<T>> + 'a>(Wrapper<Cow<'a, T>>);

fn main() {}
