#![feature(associated_consts)]

trait Tr {
    const C: Self;
}

fn main() {
    let a: u8 = Tr::C;
}
