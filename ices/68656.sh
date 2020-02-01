#!/bin/bash

rustc -C debuginfo=2 - << END
#![feature(generic_associated_types)]

trait UnsafeCopy<T: Copy>  {
    type Item<'a>: std::ops::Deref<Target = T>;

    fn bug<'a>(item: &Self::Item<'a>) -> () {
        let x: T = **item;
    }
}

impl <T: Copy> UnsafeCopy<T> for T {
    type Item<'a> = T;
}

fn main() {
    <&'static str>::bug(&"");
}

END
