#![feature(box_syntax)]
fn main() {
    let _: Box<[isize]> = box { loop {} };
}
