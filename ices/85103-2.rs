#![feature(rustc_attrs)]

trait Foo {
    type Bar;
}

#[rustc_layout(debug)]
type Edges<E> = <E as Foo>::Bar;

