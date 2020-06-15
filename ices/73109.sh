#/bin/bash

rustc -Z validate-mir - <<EOF
fn main() {
	let x = "a".to_string();
}
EOF
