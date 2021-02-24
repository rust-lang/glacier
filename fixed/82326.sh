#!/bin/bash

rustc -Zunpretty=mir-cfg - <<'EOF'
const TAG: &'static [u8] = b"ABCD";

fn main() {
    match &[][..] {
        TAG => println!("Yes."),
        _ => println!("No."),
    }
}
EOF
