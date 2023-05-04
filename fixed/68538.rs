#![feature(unsized_fn_params)]
#![crate_type = "lib"]

pub fn take_unsized_slice(s: [u8]) {
    s[0];
}
