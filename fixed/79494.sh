#!/bin/bash

cat > out.rs <<'EOF'

const ZST1: &[u8] = unsafe { std::mem::transmute(1usize) };
pub const ZST2: u8 = std::mem::transmute(1usize);

EOF

rustdoc --edition=2021 out.rs
