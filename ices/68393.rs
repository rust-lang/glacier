pub enum EFoo { A }

pub trait Foo {
    const X: EFoo;
}

struct Abc;

impl Foo for Abc {
    const X: EFoo = EFoo::A;
}

struct Def;
impl Foo for Def {
    const X: EFoo = EFoo::A;
}

pub fn test<A: Foo, B: Foo>(arg: EFoo) {
    let A::X = arg;
}

fn main() {}
