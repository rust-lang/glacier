#!/bin/sh

rustc --edition=2021 -Cincremental=93470 -Zincremental-verify-ich=yes  - << EOF

#[derive(PartialEq, Eq)]
pub struct Key {
    path: &'static str,
}

pub const CONST_A: Key = Key {
    path: "time_zone/formats@1",
};

pub const CONST_B: Key = Key {
    path: "time_zone/formats@1",
};

fn foo(key: Key) -> Result<(), &'static str> {
    match key {
        CONST_B => Ok(()),
        _ => Err(""),
    }
}

fn bar(key: Key) -> Result<(), &'static str> {
    match key {
        CONST_A => Ok(()),
        _ => Err(""),
    }
}

pub fn main() {}

EOF
