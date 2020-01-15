// reproducer by nikomatsakis
use std::marker::PhantomData;

use std::mem::{self, MaybeUninit};

struct Bug<S> {
    A: [(); {
        let x: Option<Box<S>> = None;
        0
    }],
}
