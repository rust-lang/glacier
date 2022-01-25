#![feature(generic_associated_types)]

pub trait Fooey: Sized {
    type Context<'c> where Self: 'c;
    /* // workaround
    type CallMe; */

    // fn fail2(callback: impl for<'c> Fn(&mut Self::Context<'c>)) -> Box<dyn for<'c> Fn(&mut Self::Context<'c>)> {
    //     Box::new(callback)
    // }
}

/* // workaround
pub struct Handle<E: Fooey>(Option<E::CallMe>); */
pub struct Handle<E: Fooey>(Option<Box<dyn for<'c> Fn(&mut E::Context<'c>)>>);

fn tuple<T>() -> (Option<T>,) { (Option::None,) }

pub struct FooImpl {}
impl Fooey for FooImpl {
    type Context<'c> = &'c ();
    /* // workaround
    type CallMe = Box<dyn for<'c> Fn(&mut Self::Context<'c>)>; */
}

impl FooImpl {
    pub fn fail1() -> Handle<Self> {
        let (tx,) = tuple();
        Handle(tx)
    }
}

fn main() {}
