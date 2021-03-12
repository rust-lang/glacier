#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

use std::sync::Arc;

pub enum Predicate<const EXPRESSION: bool> {}
pub trait Satisfied {}
impl Satisfied for Predicate<true> {}

pub trait Depth { const DEPTH: usize; }

/// Make it impossible to have cyclic Arc references.
pub struct AcArc<T: Depth, const D: usize> {
    #[allow(dead_code)]
    inner: Arc<T>,
}

impl<T: Depth, const D: usize> AcArc<T, D> {
    pub fn new(item: T) -> Self
        where Predicate<{T::DEPTH <= D}>: Satisfied, // crashes with == too.
    {
        Self {
            inner: Arc::new(item),
        }
    }
}

// With the predicate on the impl instead of on the function, it gives a "normal" error.
// Though I still feel like main() should typecheck.
// impl<T: Depth, const D: usize> AcArc<T, D>
//     where Predicate<{T::DEPTH == D}>: Satisfied,
// {
//     pub fn new(item: T) -> Self
//     {
//         Self {
//             inner: Arc::new(item),
//         }
//     }
// }

// this is where the depth increment is enforced.
// But compiler crashes without this.
// impl<T, const D: usize> Depth for AcArc<T, D>
// where
//     T: Depth,
// {
//     const DEPTH: usize = D + 1;
// }

impl Depth for u8 { const DEPTH: usize = 0; }

fn main() {
    let arc : AcArc<u8, 0> = AcArc::new(42_u8); // comment this out and it compiles just fine.

    println!("Hello, world!");
}
