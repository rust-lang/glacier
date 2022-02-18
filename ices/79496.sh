#!/bin/bash

cat > out.rs <<'EOF'

#![crate_type="proc-macro"]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive()]
pub fn some_derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
}


EOF

rustdoc --edition=2021 out.rs
