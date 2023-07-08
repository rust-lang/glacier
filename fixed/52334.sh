#!/bin/sh

rustc -Z mir-opt-level=0 - << EOF
type Foo = extern "C" fn(::std::ffi::CStr);
extern "C" {
    fn meh(blah: Foo);
}

fn main() {
    meh as usize;
}
EOF
