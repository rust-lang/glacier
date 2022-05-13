#![feature(generic_const_exprs)]

trait Padding {
    const PADDING: usize;
}

trait Pad: Padding
where
    [u8; Self::PADDING]:,
{
    // const VAL: [u8; Self::PADDING]; // - works
    const VAL: [u8; Self::PADDING] = [0; Self::PADDING]; // - doesn't work
}

fn main() {}
