use std::marker::PhantomData;

enum Bug {
    V = [PhantomData; { [ () ].len() ].len() as isize,
    //                ^
    //                + ---- this brace is never closed!!!
}

fn main() {}
