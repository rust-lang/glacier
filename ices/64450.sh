#!/bin/bash

mkdir debug-inconsistence && cd debug-inconsistence

mkdir -p npos-reward-curve/src runtime/src

cat > npos-reward-curve/Cargo.toml <<EOF
[package]
name = "srml-staking-npos-reward-curve"
version = "2.0.0"
authors = ["Parity Technologies <admin@parity.io>"]
edition = "2018"

[lib]
proc-macro = true

[dependencies]
EOF

cat > npos-reward-curve/src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn build(input: TokenStream) -> TokenStream {
	input
}
EOF

cd runtime

cat > Cargo.toml <<EOF
[package]
name = "node-runtime"
version = "2.0.0"
authors = ["Parity Technologies <admin@parity.io>"]
edition = "2018"

[dependencies]
curve = { package = "srml-staking-npos-reward-curve", path = "../npos-reward-curve"}
EOF

echo "curve::build! {}" > src/lib.rs

cargo check --target=wasm32-unknown-unknown
