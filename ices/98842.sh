#!/bin/bash

rustc --crate-type lib --edition=2021 - <<'EOF'

struct Foo(<&'static Foo as ::core::ops::Deref>::Target);
const _: *const Foo = 0 as _;

EOF

