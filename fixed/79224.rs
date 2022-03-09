#![feature(min_specialization)]
use std::fmt::{self, Display};

pub enum Cow<'a, B: ?Sized + 'a, O = <B as ToOwned>::Owned>
where
    B: ToOwned,
{
    Borrowed(&'a B),
    Owned(O),
}


impl ToString for Cow<'_, str> {
    fn to_string(&self) -> String {
        panic!()
    }
}

impl<B: ?Sized> Display for Cow<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!()
    }
}
