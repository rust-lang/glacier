use std::pin::Pin;

fn main() {
    let a = Enum::A(Pin::new(Box::new(A())));
    let b = Enum::B(Pin::new(Box::new(B())));
    println!("{:?} {:?}", a, b);
}

#[derive(Debug)]
enum Enum {
    A(Pin<Box<A>>),
    B(Pin<Box<B>>),
}

#[derive(Debug)]
struct A();

impl Drop for Pin<Box<A>> {
    fn drop(&mut self) {}
}

#[derive(Debug)]
struct B();

// UNCOMMENT TO FIX COMPILER ERROR
// impl Drop for Pin<Box<B>> {
//     fn drop(&mut self) {}
// }
