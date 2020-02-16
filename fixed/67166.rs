#![feature(impl_trait_in_bindings)]

#[allow(dead_code)]
fn run() {
    let _foo: Box<impl Copy + '_> = Box::new(());
}

fn main() {}
