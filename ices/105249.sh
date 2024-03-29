#!/bin/bash

rustc -Zpolymorphize=on - << EOF
trait Foo<T> {
    fn print<'a>(&'a self) where T: 'a { println!("foo"); }
}

impl<'a> Foo<&'a ()> for () { }

trait Bar: for<'a> Foo<&'a ()> { }

impl Bar for () {}

fn main() {
    (&() as &dyn Bar).print(); // Segfault
}

EOF
