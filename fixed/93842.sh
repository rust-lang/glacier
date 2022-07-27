#!/bin/bash

rustc --target x86_64-pc-windows-msvc - <<'EOF'
#![feature(raw_dylib)]
#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    fn WaitForSingleObject(handle: isize, dwmilliseconds: u32) -> u32;
}
fn main() {
    unsafe { WaitForSingleObject(0, 0); }
    println!("ok");
}
EOF
