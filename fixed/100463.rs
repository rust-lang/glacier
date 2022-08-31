struct Foo<T> {
    inner: Vec<T>,
}

impl<T> Foo<T> {
    fn get(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

fn main() {
    let foo: Foo<()> = Foo { inner: Vec::new() };
    let vals: Vec<_> = foo.get();
}
