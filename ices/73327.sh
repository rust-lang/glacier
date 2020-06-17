#!/bin/bash

rustc -Z mir-opt-level=2 - << EOF

fn load<R>(_r: R) {}

#[inline(always)]
pub fn load_from_memory_with_format(buf: &[u8]) {
    let b = ::std::io::Cursor::new(buf);
    load(b)
}

fn main() {
    load_from_memory_with_format(&[72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]);
}
EOF
