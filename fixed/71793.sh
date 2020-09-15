#!/bin/bash

rustc +nightly --edition 2018 -Z mir-opt-level=2 - << EOF
pub async fn connect() {}
pub fn block_on<F: std::future::Future>(_: F) {}
fn main() {
    block_on(async {
        Vec::<String>::new().first().ok_or("").unwrap();
        connect().await;
    })
}

EOF
