struct Foo;

trait MyTrait {
    type Item<T>;
}

impl MyTrait for Foo {
    type Item<T> = T;
}
