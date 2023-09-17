union Foo {
    a: isize,
    b: &VTable,
}
enum Bar {
    Boo = [Foo { b: () }.a][3],
}
