#![feature(or_patterns)]

fn f(s: &str, num: usize) {
    match (s, num) {
        ("", 0) | ("a" | "b", 1) => (),
        _ => (),
    }
}

fn main() {}
