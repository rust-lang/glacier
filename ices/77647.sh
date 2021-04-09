#!/bin/bash

cat > out.rs <<'EOF'
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]
struct A<T, const N: core::num::NonZeroUsize>([T; N.get()]) where [T; N.get()]: Sized;
EOF

rustdoc out.rs -Z unstable-options --output-format json
