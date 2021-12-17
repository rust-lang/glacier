#![feature(unboxed_closures)]

struct ErrorKind;
struct Error(ErrorKind);
impl Fn(&isize) for Error {
    fn from() {}
}

fn main() {}
