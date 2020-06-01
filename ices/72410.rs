pub trait Foo {
    fn map() where Self: Sized, for<'a> &'a mut [u8] : ;
}
