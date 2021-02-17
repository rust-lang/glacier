union Transmute<T: Copy, U: Copy> {
    t: T,
    u: U,
}
trait Bar {
    fn bar(&self) -> bool;
}
struct Foo {
    foo: bool,
    bar: bool,
}
impl Bar for Foo {
    fn bar(&self) -> bool {
        self.foo
    }
}
#[derive(Copy, Clone)]
struct Fat<'a>(&'a Foo, &'static VTable);
struct VTable {
    size: Foo,
}
const FOO: &Bar = &Foo {
    foo: true,
    bar: false,
};
const G: Fat = unsafe { Transmute { t: FOO }.u };

fn main() {}
