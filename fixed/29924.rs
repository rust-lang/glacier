#![feature(const_fn, associated_consts)]

trait Trait {
    const N: usize;
}

impl Trait {
    const fn n() -> usize { Self::N }
}

fn main() {
}
