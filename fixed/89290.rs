#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]

const fn foo<T: ~const Drop>(_: T) {}

const fn bar() {
    foo(|| ());
}

pub fn main() {}
