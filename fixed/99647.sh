#!/bin/bash

rustc --edition=2018 -Zcrate-attr="feature(generic_const_exprs)" - <<'EOF'

#[allow(unused)]
async fn foo<'a>() {
    let _data = &mut [0u8; { 1 + 4 }];
    bar().await
}

async fn bar() {}

fn main() {}

EOF
