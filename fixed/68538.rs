#![feature(unsized_locals)]
#![crate_type = "lib"]

pub fn take_unsized_slice(s: [u8]) {
    s[0];
}
