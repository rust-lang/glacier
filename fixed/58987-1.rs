use std::mem;

fn foo<T: ?Sized>(t: T) {
    let _ = [(); 0 - !(mem::size_of::<&T>() == mem::size_of::<[usize; 2]>()) as usize];
}

fn main() {}
