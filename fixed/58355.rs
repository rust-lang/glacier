#![crate_type = "lib"]

pub fn foo(callback: fn() -> ToString) {
    let mut x: Option<Box<Fn() -> ToString>> = None;
    x = Some(Box::new(callback));
}
