#!/bin/bash

cat > x.rs <<EOF
#![crate_type = "lib"]

#[inline(always)]
pub fn f(s: bool) -> String {
    let a = "Hello world!".to_string();
    let b = a;
    let c = b;
    if s {
        c
    } else {
        String::new()
    }
}

EOF

cat > y.rs <<EOF
#![crate_type = "lib"]

pub async fn g() {
    x::f(true);
    h().await;
}

pub async fn h() {}

EOF

rustc --edition=2018 -Zmir-opt-level=3 -Zunsound-mir-opts x.rs
rustc --edition=2018 -Zmir-opt-level=3 y.rs --extern x -L.
