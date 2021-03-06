#![crate_type = "lib"]
#![feature(inline_const)]

fn foo<const V: usize>() {
  match 0 {
    const { V } => {},
    _ => {},
  }
}
