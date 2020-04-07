#![feature(const_generics)]
fn test<const A: i32>() -> i32 { A }

trait T {
    fn test<const A: i32>(&self) -> i32 { A }
}

struct S();

impl T for S {}

fn main() {
  let foo = S();
  println!("{}", test::<{8i32}>());
  println!("{}", foo.test::<{16i32}>());  // <- Causes ICE
}
