#![feature(const_generics)]

struct Caca<const A: &'static str> {
    s: String
}

impl<const A: &str> Default for Caca<A> {
    fn default() -> Self {
        let a = Self::A;
        Self {
            s: A.to_string()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
