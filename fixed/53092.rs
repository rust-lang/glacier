#![feature(min_type_alias_impl_trait)]
#![allow(dead_code)]

type Bug<T, U> = impl Fn(T) -> U + Copy;

const CONST_BUG: Bug<u8, ()> = unsafe { std::mem::transmute(|_: u8| ()) };

fn make_bug<T, U: From<T>>() -> Bug<T, U> {
    |x| x.into()
}

fn main() {
    CONST_BUG(0);
}
