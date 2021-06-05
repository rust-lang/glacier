#!/bin/bash

cat << BLOCK > Cargo.toml
[package]
name = "abc"
version = "0.1.0"
edition = "2018"
BLOCK

mkdir -p src tests

cat << BLOCK > build.rs
fn main() {
    std::fs::write(
        &std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("crash.rs"),
        "pub struct A;",
    )
    .unwrap();
}
BLOCK

cat << BLOCK > src/lib.rs
include!(concat!(env!("OUT_DIR"), "/", "crash.rs"));
BLOCK

cat << BLOCK > tests/crash.rs
extern crate abc;
include!(concat!(env!("OUT_DIR"), "/", "crash.rs"));
BLOCK

cargo test
