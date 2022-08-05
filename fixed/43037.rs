#![crate_type = "lib"]
#![feature(specialization)] // comment this line to get the expected behavior
trait X {}
trait Y: X {}
trait Z { type Assoc: Y; }
struct A<T>(T);

impl<T> Y for T where T: X {}
impl<T: X> Z for A<T> { type Assoc = T; }

// this impl is invalid, but causes an ICE anyway
impl<T> From<<A<T> as Z>::Assoc> for T {}

fn main() {}
