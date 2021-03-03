#![feature(const_panic)]

const C: () = {
    panic!(1234);
};

fn main() {}
