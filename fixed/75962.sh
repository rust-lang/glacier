#!/bin/bash

rustc -Z save-analysis - << EOF

#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

struct Foo<T>(T);

trait FooLike { type Output; }

impl<T> FooLike for Foo<T> {
    type Output = T;
}


// Reduction using `impl Trait` in bindings

mod impl_trait_in_bindings {
    struct Foo;

    trait FooLike { type Output; }

    impl FooLike for Foo {
        type Output = u32;
    }

    trait Trait {
        type Assoc;
    }

    fn foo<T: Trait<Assoc=u32>>() {
        let _: impl FooLike<Output=T::Assoc> = Foo;
    }
}

fn main() {}

EOF
