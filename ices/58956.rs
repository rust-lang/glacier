#![feature(impl_trait_in_bindings)]

trait Lam {}

pub struct B;
impl Lam for B {}
pub struct Wrap<T>(T);

const _A: impl Lam = {
    let x: Wrap<impl Lam> = Wrap(B);
    x.0
};

fn main() {}
