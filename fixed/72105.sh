#!/bin/bash

cat > 72105.rs <<EOF
extern "C" {
    static _dispatch_queue_attr_concurrent: [u8; 0];
}

fn main() {
let x: &[u8; 0] =
    unsafe { &_dispatch_queue_attr_concurrent };
}
EOF

rustc --emit=mir 72105.rs
