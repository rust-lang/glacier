#![feature(associated_consts)]

struct S;

impl S {
    const N: usize = 3;
}

static STUFF: [u8; S::N] = [0; S::N];

// NOTE: This works OK:
// static STUFF: [u8; 3] = [0; S::N];

fn main() {
    // NOTE: This works OK:
    // let stuff: [u8; S::N] = [0; S::N];
}
