#!/bin/sh

rustc --edition=2021 - << EOF

struct Struct<'a>(&'a str);

async fn foo() -> Struct {
    todo!()
}

fn main() {
    foo();
}

EOF
