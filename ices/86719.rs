trait Bar {
    type E;
}
impl<S> Bar for S {
    type E = impl ;
    fn foo() -> Self::E {
        |_| true
    }
}
