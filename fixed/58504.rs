#![feature(never_type)]
#![feature(impl_trait_in_bindings)]
#![feature(generators,generator_trait)]

use std::ops::Generator;

fn mk_gen() -> impl Generator<Return=!, Yield=()> {
    || { loop { yield; } }
}

fn main() {
    let gens: [impl Generator<Return=!, Yield=()>;2] = [ mk_gen(), mk_gen() ];
}
