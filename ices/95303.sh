rustc  --crate-type lib  - 2>&1 << EOF


#![feature(no_core)]
#![feature(lang_items)]

#![no_core]

#[no_mangle]
pub fn add(a: u32) -> u32 {
    a
}

#[lang = "sized"]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy {}


EOF
