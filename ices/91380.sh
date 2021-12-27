#!/bin/bash

cat > foo.rs << 'EOF'
pub trait Module {
    type Algebra;
}

pub trait ChainComplex {
    type Algebra;
    type Module: Module;
}

pub struct FreeModule<A> {
    foo: A,
}

impl<A> Module for FreeModule<A> {
    type Algebra = A;
}

pub trait FreeChainComplex:
    ChainComplex<Module = FreeModule<<Self as ChainComplex>::Algebra>>
{
}

pub struct ResolutionHomomorphism<CC2>
where
    CC2: ChainComplex,
{
    maps: CC2::Module,
}

pub struct SecondaryResolutionHomomorphism<
    CC2: FreeChainComplex<Algebra = usize>,
> {
    underlying: ResolutionHomomorphism<CC2>,
}
EOF

rustdoc foo.rs