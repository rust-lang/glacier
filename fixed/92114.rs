#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]
#![feature(inline_const)]
#![feature(negative_impls)]

const fn const_drop<T: ~const Drop>(_x: T) {}

struct NonDrop;

impl !Drop for NonDrop {}

fn main() {
    const { const_drop(NonDrop) };
}
