#![feature(const_generics)]

#[derive(PartialEq, Eq)]
struct Config {
    arr_size: usize
}

struct B<const CFG: Config> {
    arr: [u8; {CFG.arr_size}]
}

fn main() {}
