#![feature(const_generics)]

pub fn function_with_str<'a, const STRING: &'a str>() {
}

pub fn use_it() {
    function_with_str::<"Hello, world!">()
}

fn main() {}
