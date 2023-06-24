#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

struct Closure;

impl const FnOnce<&usize> for Closure {
    type Output = usize;

    extern "rust-call" fn call_once(self, arg: &usize) -> Self::Output {
        *arg
    }
}

enum Bug<T = [(); Closure.call_once(&0) ]> {
    V(T),
}

fn main() {}
