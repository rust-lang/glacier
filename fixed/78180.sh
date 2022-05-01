#!/bin/bash

rustc -Zsave-analysis - <<EOF
#![feature(min_const_generics)]

fn test<const N: usize>() {}

fn wow<'a>() -> &'a () {
    test::<
        {
            let _: &'a ();
            3
        },
    >();
    &()
}
EOF
