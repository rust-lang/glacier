#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use core::fmt::Debug;

trait Foo {
    async fn baz()-> impl Debug{}
}

fn main(){}
