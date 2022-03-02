#![feature(type_alias_impl_trait, generator_trait, generators)]
use std::ops::Generator;

trait Runnable {
    type Gen: Generator<Yield = (), Return = ()>;

    fn run(&mut self) -> Self::Gen;
}

struct Implementor {}

impl Runnable for Implementor {
    type Gen = impl Generator<Yield = (), Return = ()>;

    fn run(&mut self) -> Self::Gen {
        move || {
            // This line causes the ICE;
            // the same error occurs if the return type is wrong (e.g. `return 1`)
            yield 1;
        }
    }
}
