trait A {
    type B<'b>;
    fn a() -> Self::B<'static>;
}

struct C;

struct Wrapper<T>(T);

impl A for C {
    type B<T> = Wrapper<T>;
    fn a() -> Self::B<'static> {}
}
