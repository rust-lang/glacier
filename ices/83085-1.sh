#!/bin/bash

rustc -Z incremental-verify-ich=yes -C incremental=foo - <<'EOF'
fn main() {
    const BOO: &[u8; 0] = &[];
    match &[] {
        BOO => (),
        b"" => (),
        _ => (), 
    }
}
EOF
