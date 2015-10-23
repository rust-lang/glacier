trait Foo<T:?Sized> {type S;}
trait Hack: Foo<Self> {}
fn takes_hack(x: &Hack<S=()>) {}
fn main() {}
