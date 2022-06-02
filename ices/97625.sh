#!/bin/sh

rustc -Cinstrument-coverage - << EOF

#![feature(adt_const_params)]
#![allow(incomplete_features)]

fn eval<const N: f32>() -> f32 {
  N
}

fn main() {
  println!("{}", eval::<0.5>());
}

EOF
