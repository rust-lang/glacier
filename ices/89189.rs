#![feature(fn_traits, unboxed_closures)]
struct Func {}
impl Fn(&str) -> bool for Func {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        todo!()
    }
}

fn main() {}
