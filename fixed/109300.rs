#![feature(generic_const_exprs)]
trait B {
    type U<T: A>;
}

fn f<T: B<U<1i32> = ()>>() {}

fn main() {}
