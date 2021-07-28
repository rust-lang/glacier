trait Foo<T, T = T> {}
fn eq<A, B>() {
    eq::<dyn, Foo>
}
