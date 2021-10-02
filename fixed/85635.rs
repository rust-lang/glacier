#![feature(generators)]
#![feature(box_syntax)]

fn main() {
    || box yield;
}
