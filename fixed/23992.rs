trait Foo {
    type Bar;
    fn foo(self) -> Self::Bar;
}

impl Foo for Box<Foo> {
    type Bar = <Self as Foo>::Bar;
    fn foo(self) -> <Self as Foo>::Bar {
        (*self).foo()
    }
}
