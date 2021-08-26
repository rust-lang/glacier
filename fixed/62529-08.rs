use std::marker::PhantomData;

trait Foo<'a> {
    type Item;
    fn consume<F>(self, f: F) where F: Fn(Self::Item);
}
struct Consume<A>(PhantomData<A>);

impl<'a, A:'a> Foo<'a> for Consume<A> {
    type Item = &'a A;

    fn consume<F>(self, _f: F) where F: Fn(Self::Item) {
        if blackbox() {
            _f(any()); // Gotta keep this (1.)
        }
    }
}

#[derive(Clone)]
struct Wrap<T> { foo: T }

impl<T: for <'a> Foo<'a>> Wrap<T> {
    fn consume<F>(self, f: F) where F: for <'b> Fn(<T as Foo<'b>>::Item) {
        self.foo.consume(f);
    }
}

fn main() {
    // This works
    Consume(PhantomData::<u32>).consume(|item| { let _a = item; });

    // This does not (but is only noticed if you call the closure).
    let _wrap = Wrap { foo: Consume(PhantomData::<u32>,) };
    _wrap.consume(|item| { let _a = item; }); // Gotta keep this (2.)
}

pub static mut FLAG: bool = false;
fn blackbox() -> bool { unsafe { FLAG } }
fn any<T>() -> T { loop { } }
