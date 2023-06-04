#!/bin/bash

rustc -Cincremental=foo - <<'EOF'
#[derive(PartialEq, Eq)]
pub struct StaticString {
    string: &'static str,
}

const DUMMY1: StaticString = StaticString {
    string: "DUMMY",
    };
const DUMMY2: StaticString = StaticString {
    string: "DUMMY",
    };

fn main() {
    let bar = DUMMY1;
    let name = match bar {
            DUMMY1 => "1",
            DUMMY2 => "2",
            _ => "Foo",
    };
}
EOF
