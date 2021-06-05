#![feature(const_panic)]

const fn hey() -> usize {
    panic!(123);
}

fn main() {
    let _: [u8; hey()] = todo!();
}
