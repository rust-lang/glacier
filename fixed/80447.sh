#!/bin/bash

rustc --crate-type=proc-macro - << 'EOF'
extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro_attribute]
pub fn mac(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}
EOF

rustc --extern mac=$(ls librust_out.*) - << 'EOF'
pub trait Crash {
    #[mac::mac]
    fn one(s: () {
    }

    fn two();
}

fn main () {}
EOF
