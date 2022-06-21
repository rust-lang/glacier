pub trait Trait: {}

pub struct Struct<T: Trait> {
    member: T,
}

// uncomment and bug goes away
//impl Trait for u8 {}

extern "C" {
    static VAR: Struct<u8>;
}

fn main() {}
