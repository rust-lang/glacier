#![crate_type = "lib"]
pub trait Foo {
    fn do_stuff() -> Self;
}

pub trait Bar {
    type Output;
}

impl<T> Foo for dyn Bar<Output = T>
where
    Self: Sized,
{
    fn do_stuff() -> Self {
        todo!()
    }
}
