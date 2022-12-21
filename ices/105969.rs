#![feature(dyn_star)]
#![allow(incomplete_features)]

use core::fmt::Debug;

const FOO: () = {
    let foo = &3;
    let i = foo as dyn* Debug;
};

fn main() {}
