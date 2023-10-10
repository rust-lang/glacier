struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Subtype = Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>;
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo() -> impl Sized {
    loop {
        match foo() {
            Subtype::Bar => (),
            Supertype::Var(x) => {}
        }
    }
}

fn main() {}
