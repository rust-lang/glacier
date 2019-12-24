#![feature(or_patterns)]

fn foo((Some(_) | None): Option<u32>) {}

fn main() {}
