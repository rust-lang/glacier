#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub trait TraitWAssocConst {
    const A: dyn TraitWAssocConst<A = 0>;
}

fn bar<A: TraitWAssocConst<A = 0>>() {}

fn main() {}
