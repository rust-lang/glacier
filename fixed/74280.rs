#![feature(type_alias_impl_trait)]

type Test = impl Copy;

fn test() -> Test {
    let y = || -> Test { () };
    7
}

fn main() {}
