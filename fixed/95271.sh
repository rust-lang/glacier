rustc  --crate-type lib --edition=2021 - 2>&1 << EOF

enum Foo {
    Foo(i32),
}

fn bar(foo: Foo) {
    || {
        // `let foo = foo;` makes the ICE disappear
        let Foo::Foo(baz) = foo;
    };
}
pub fn main() {}

EOF
