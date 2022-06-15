#![recursion_limit = "1000"]

use std::marker::PhantomData;

struct Z;
struct S<T>(PhantomData<T>);

type Nested = S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<S<Z>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;

trait AsNum {
    const NUM: u32;
}

impl AsNum for Z {
    const NUM: u32 = 0;
}

impl<T: AsNum> AsNum for S<T> {
    const NUM: u32 = T::NUM + 1;
}

fn main() {
    println!("{}", Nested::NUM);
}
