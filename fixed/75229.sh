#!/bin/bash

cat > 75229.rs <<EOF
enum TestEnum {
    Field { test_field: String },
}
fn main(){
	()
}

EOF

RUST_BACKTRACE=1 RUST_SAVE_ANALYSIS_CONFIG='{"output_file":null,"full_docs":true,"pub_only":false,"reachable_only":false,"distro_crate":false,"signatures":true,"borrow_data":false}' rustc 75229.rs -Z save_analysis
