#![feature(non_lifetime_binders)]

trait Trait<Input> {}

fn main(x: impl for<f> Trait<'_, Assoc = impl Trait<f> + '_>) {}
