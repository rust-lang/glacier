trait Trait<E> {
    type Assoc;
}

struct Foo;

impl<'a> Trait<&'a ()> for Foo {
    type Assoc = ();
}

fn foo() -> impl for<'a> Trait<&'a ()> {
    Foo
}

fn bar() -> impl for<'a> Trait<&'a (), Assoc = impl Sized> {
    foo()
}
