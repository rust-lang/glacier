pub enum Foo<const A: usize> {
    Bar,
    Baz([(); A]),
}


fn main() {
    // Cannot infer type for 9
    // let x = Foo::Bar::<9>;
    // Compiler error
    let x = Foo::Bar::<9usize>;
}
