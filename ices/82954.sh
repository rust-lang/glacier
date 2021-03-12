#!/bin/bash

rustc -C debuginfo=2 - <<'EOF'
use std::marker::PhantomData;

pub enum Empty {}

fn main() {
    let _x: PhantomData<(Empty, [usize])>;
}
EOF
