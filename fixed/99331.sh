#!/bin/bash

rustc -Zunpretty=expanded - <<'EOF'

extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI

fn main() { }

EOF

