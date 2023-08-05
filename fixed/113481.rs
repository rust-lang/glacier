trait Foo {
    type Bar<'x>
    where
        Self: 'x;
}

fn baz() -> impl for<'y> Foo<Bar<'y> = impl ToString> {
    42
}

impl Foo for i32 {
    type Bar<'x> = &'x str;
}
