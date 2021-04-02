struct Foo {}
impl Foo {
    fn bar(foo: Foo<Target = usize>) {}
}
fn main() {}
