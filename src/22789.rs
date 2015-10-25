#![feature(core)]
#![feature(unboxed_closures)]

fn main() {
    let k = |x: i32| { x + 1 };
    Fn::call(&k, (0,));
}
