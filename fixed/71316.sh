#!/bin/bash

rustc -Zunleash-the-miri-inside-of-you - << END

#![feature(const_raw_ptr_deref)]
#![feature(const_mut_refs)]
#![allow(const_err)]

use std::cell::UnsafeCell;

// make sure we do not just intern this as mutable
const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;

fn main() {}

END
