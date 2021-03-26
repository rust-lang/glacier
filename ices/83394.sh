#!/bin/bash

rustc -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type lib - <<'EOF'
#![feature(rustc_attrs)]

// Uncommenting any of these attributes will successfully compile 
#[repr(C)]
#[derive(Debug)]
#[rustc_layout_scalar_valid_range_end(0x7F)]
struct Integer(u8);


// triggers different compiler panic
/*
#[repr(C)]
#[derive(Debug)]
#[rustc_layout_scalar_valid_range_end(0xFFFF)]
struct Integers(u8, u8);
*/
EOF
