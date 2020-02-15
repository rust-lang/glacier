#![feature(generic_associated_types)]

trait Fun {
    type F<'a: 'a>;
}

impl <T> Fun for T {
    type F<'a> = Self;
}


fn main() {
}
