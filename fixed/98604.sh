#!/bin/bash

rustc --edition=2021 - <<'EOF'

type AsyncFnPtr = Box<
    dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
>;

async fn test() {}

#[allow(unused_must_use)]
fn main() {
    Box::new(test) as AsyncFnPtr;
}

EOF

