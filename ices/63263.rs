#![feature(type_alias_impl_trait)]

pub type Closure = impl FnOnce();

fn main() {
    || -> Closure { || () };
}
