#![feature(associated_type_bounds)]

struct Bug<T: ?Sized>(T);

impl Bug<dyn Iterator<Item: Copy>> {}

fn main() {}
