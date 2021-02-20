#!/bin/bash

rustc -Cpanic=abort --crate-type proc-macro --crate-name=p - <<'EOF'
extern crate proc_macro;

#[proc_macro]
pub fn p(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic!()
}
EOF

rustc -L. - <<'EOF'
extern crate p;
p::p! {}
EOF
