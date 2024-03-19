pub trait TraitWAssocConst<T> {
    const A: T;
}

fn foo<T, B: TraitWAssocConst<T, A = { 1 }>>() {}

fn main() {}
