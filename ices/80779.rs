pub struct T<'a>(&'a str);

pub fn f<'a>(val: T<'a>) -> _ {
  g(val)
}

pub fn g(_: T<'static>) -> _ {}

fn main() {}
