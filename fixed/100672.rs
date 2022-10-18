#![feature(generic_associated_types)]

trait Bar<'a> {
    type Ref<'b>
    where
        'a: 'b;
    fn uwu(f: impl Fn(Self::Ref<'_>));
}

impl<'a> Bar<'a> for () {
    type Ref<'b> = () where 'a: 'b;
    fn uwu(f: impl Fn(())) {}
}

pub fn main() {}
