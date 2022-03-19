#!/usr/bin/env bash

rustc --target x86_64-pc-windows-msvc -C debuginfo=1 - <<EOF
use std::mem::MaybeUninit;

unsafe fn make_fat_ptr<T: ?Sized>(_: &u32) -> *mut T {
    MaybeUninit::<*mut T>::uninit().assume_init()
}

struct VNode<'src> {
    _x: &'src [VNode<'src>]
}

fn main() {
    let g: *mut dyn Fn(&'_ u32) -> Option<VNode<'_>> =
        unsafe {make_fat_ptr(&12)};
}
EOF