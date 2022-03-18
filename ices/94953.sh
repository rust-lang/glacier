rustc  -Zunpretty=expanded - 2>&1 << EOF

#![feature(lint_reasons)]
#![warn(unused)]

// This expect attribute should catch all lint triggers
#[expect(while_true)]
fn check_multiple_lints_3() {
    // `while_true` is an early lint
    while true {}
}

fn main() {
    check_multiple_lints_3();
}

EOF
