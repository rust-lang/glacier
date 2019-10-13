use std::marker::PhantomData;

struct Invariant<'id, I = Self> {
    lifetime: PhantomData<*mut &'id I>,
}

trait Contract<'s> {
    type Item: for<'r> Contract<'r>;
}

impl <'a, 'b> Contract<'b> for Invariant<'a> {
    type Item = Invariant<'b>;
}

fn main() {}
