#!/bin/bash

mkdir proc-macro-ICE && cd proc-macro-ICE

mkdir -p proc-macro-bug-impl/src proc-macro-bug/src

cat > Cargo.toml <<EOF
[workspace]

members = [
    "proc-macro-bug",
    "proc-macro-bug-impl"
]
EOF

cat > proc-macro-bug-impl/Cargo.toml <<EOF
[package]
name = "proc-macro-bug-impl"
version = "0.0.1"
authors = ["olegnn <olegnosov1@gmail.com>"]
edition = "2018"
license = "MIT"

[lib]
name = "proc_macro_bug_impl"
proc-macro = true

[features]
std = []

[dependencies]
quote = "1.0.2"
syn = "1.0"
EOF

cat > proc-macro-bug-impl/src/lib.rs <<EOF
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, Path};

thread_local! {
    static DEFAULT_CRATE_PATH: Path = parse_quote! { ::std };
}

#[proc_macro_attribute]
pub fn some_macro(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::from(DEFAULT_CRATE_PATH.with(|default_crate_path| {
        quote! {
            use #default_crate_path::boxed::Box;
        }
    }))
}
EOF

cat > proc-macro-bug/Cargo.toml <<EOF
[package]
name = "proc-macro-bug"
version = "0.0.1"
authors = ["olegnn <olegnosov1@gmail.com>"]
license = "MIT"
edition = "2018"

[dependencies]
proc-macro-bug-impl = { path = "../proc-macro-bug-impl", version = "0.0.1" }
EOF

cat > proc-macro-bug/src/main.rs <<EOF
extern crate proc_macro_bug_impl;
use proc_macro_bug_impl::some_macro;

#[some_macro(0)]
struct Abc {}

#[some_macro(0)]
struct Cde {}

fn main() {}
EOF

RUST_BACKTRACE=full cargo +nightly run
