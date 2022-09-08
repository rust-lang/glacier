#!/bin/bash

rustc   -Cincremental=/tmp/a -Zincremental-verify-ich=yes --crate-type lib - <<'EOF'

#[derive(Eq, PartialEq)]
struct Id(&'static str);

fn f() {
    const FLAG: Id = Id("");
    match Id("") {
        FLAG => (),
        _ => (),
    };
}

fn g() {
    const FLAG: Id = Id("");
    match Id("") {
        FLAG => (),
        _ => (),
    };
}

EOF
