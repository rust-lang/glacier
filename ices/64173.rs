use std::mem::size_of;

struct Bug<'s> {
    array: [(); size_of::<&Self>()],
}

fn main() {}
