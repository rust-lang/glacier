#![feature(let_chains)]

fn main() {
    if true && let x = 1 {
        let _ = x;
    }
}
