#![feature(type_alias_impl_trait)]

type Closure = impl Fn() -> u64;
struct Anonymous(Closure);

fn test() {
    let y = || -> Closure { || 3 };
    Anonymous(|| 3)
}

fn main() {}
