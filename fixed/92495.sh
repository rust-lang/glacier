#!/usr/bin/env bash

rustc --crate-type lib  -  << EOF
#![feature(lang_items, no_core, start)]
#![no_std]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
trait Copy {}

#[start]
fn main(argc: isize, _argv: *const *const u8) -> isize {
    match argc {
        1 => 1,
        _ => 0,
    }
}
EOF
