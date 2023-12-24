#!/bin/bash

rustc -O - << EOF

#![feature(let_chains)]

struct F(Box<()>);

impl F {
    fn s(&self) -> Option<&str> {
        None
    }
}

fn cex() -> Option<F> {
    None
}

fn main() {
    if false
        && let Some(ce) = cex()
        && let Some(_ce) = ce.s()
    {
    }
}

EOF
