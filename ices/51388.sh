#!/bin/bash

cat > bar.rs << 'EOF'
pub fn hello_world()
{
    println!("hello world");
}
EOF

cat > foo.rs << 'EOF'
extern crate bar;

pub fn foo()
{
    bar::hello_world();
}
EOF

rustc --crate-type rlib --emit metadata bar.rs
rustc --crate-type rlib --emit llvm-bc bar.rs
rustc --crate-type rlib --emit llvm-bc foo.rs -L .
