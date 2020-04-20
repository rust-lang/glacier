#![feature(const_mut_refs)]

const FOO: &mut i32 = &mut 4;

fn main() {
    *FOO = 2;
}
