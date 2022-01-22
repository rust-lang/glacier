#!/usr/bin/env bash

rustc --crate-type lib -Wrust-2021-incompatible-closure-captures - 2>&1 << EOF

pub struct A {}

impl A {
    async fn create(path: impl AsRef<std::path::Path>)  {
    ;
    crate(move || {} ).await
    }
}


EOF
