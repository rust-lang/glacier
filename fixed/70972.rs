use std::sync::atomic::{AtomicPtr, Ordering};

const P: &dyn T = &S as &dyn T;
static mut LOGGER: AtomicPtr<&dyn T> = AtomicPtr::new(&mut P);

pub trait T {}
struct S;
impl T for S {}

pub fn f() {
    match *LOGGER.load(Ordering::SeqCst) {
        P | _ => {}
    }
}

fn main() {}
