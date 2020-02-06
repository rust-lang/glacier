#!/bin/bash

tmp="$(mktemp -d)"

if [[ ! $tmp || ! -d $tmp ]]
then
    echo "Could not create temporary directory"
    exit 1
fi

cleanup() {
    rm -r "$tmp"
}

trap cleanup EXIT

cd "$tmp"

rustc --crate-type proc-macro - <<END
extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn no_main(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(vec![
        TokenTree::Ident(Ident::new("fn", Span::call_site())),
        TokenTree::Ident(Ident::new("main", Span::call_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Group(Group::new(Delimiter::Brace, input)),
    ]);
    ts
}
END

rustc --extern no_main=librust_out.so - <<END
#![no_main::no_main]
END
