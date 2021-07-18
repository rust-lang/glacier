#!/bin/bash

cat > the_file.json <<'EOF'
{
  "arch": "x86_64",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "is-builtin": true,
  "llvm-target": "x86_64-unknown-unknown-gnu",
  "target-pointer-width": "64"
}
EOF

echo '#![feature(no_core)] #![no_core]' | rustc --target the_file.json -
