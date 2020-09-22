#![feature(const_generics)]
#![allow(incomplete_features)]
#![crate_type = "lib"]

pub async fn baz<const H: usize>() {
    biz(&Vec::new()).await;
}

const SIZE: usize = 16;

pub async fn biz(_: &[[u8; SIZE]]) {}
