#![feature(lang_items, no_core)]
#![no_std]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
trait Copy {}

fn bla(argc: isize) {
    match argc {
        1 => 1, // removing this arm fixes the ICE
        _ => 1,
    };
}

fn main() {}
