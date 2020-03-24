mod foo {
    pub enum Foo {
        Foo(i32),
    }
}
use foo::*;
use Foo::Foo;
