#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "alba"
version = "0.0.1"
edition = "2021"

[dependencies]
sera = { path = "sera/" }
EOF

mkdir -p src sera/src

cat > sera/Cargo.toml <<'EOF'
[package]
name = "sera"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["proc-macro"]
EOF

cat > sera/src/lib.rs << EOF
extern crate proc_macro;

#[proc_macro]
pub fn ignore(input: TokenStream) -> TokenStream {
    let mut result : TokenStream = TokenStream::new();
    result.into()
}

#[proc_macro]
pub fn this_fails(input: TokenStream) -> TokenStream {
    let mut result : TokenStream = TokenStream::new();
    result.extend::<TokenStream>("metamodel_macros::ignore!(42).into()".parse().unwrap());
    result.into()
}
EOF

cat > src/main.rs << EOF
#[test]
pub fn bug_report() {
    sera::this_fails!(1*2*3*7);
}
EOF

cargo test
