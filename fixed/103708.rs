#![feature(min_specialization)]

trait Dance {
    fn foo();
}

impl<'a, T> Dance for T {
    fn foo() {}
}

impl Dance for bool {
    fn foo() {}
}

fn main() {}
