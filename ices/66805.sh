#!/bin/bash

cargo new pm && cd pm

cat > Cargo.toml <<EOF
[package]
name = "pm"
version = "0.0.0"
edition = "2018"

[lib]
proc-macro = true

[dependencies]
syn = { version = "1", features = ["full", "visit-mut"]}
quote = "1"
EOF

mkdir -p src

cat > src/main.rs <<EOF
#![feature(proc_macro_hygiene)]

use std::marker::PhantomData;
use pm::crash;

mod a;

struct Sender<T> {
    _p: PhantomData<T>,
}

impl<T> Sender<T> {
    fn new() -> Self { unimplemented!() }
    fn send(self, _: T) {}
}

fn func() {}

fn main() {
    stream! {
        ()?;
        yield func();
    }   
}
EOF

cat > src/a.rs <<EOF
#[macro_export]
macro_rules! stream {
    ($($body:tt)*) => {{
        crash! {
             {
                 $($body)*
             }
        }
        stream_0!()
    }}  
}
EOF

cat > src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::{Group, TokenStream, TokenTree};
use quote::quote;
use syn::visit_mut::VisitMut;

struct Dummy;


impl VisitMut for Dummy {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        match i { 
            syn::Expr::Yield(yield_expr) => {
                let value_expr = yield_expr.expr.as_ref().unwrap();
                *i = syn::parse_quote! { __yield_tx.send(#value_expr)}
            }
            syn::Expr::Try(try_expr) => {
                let e = &try_expr.expr;
                *i = syn::parse_quote! { __yield_tx.send(Err(#e)) };
            }
            _ => (), 
        }
    }   
}

fn replace_for_await(input: TokenStream) -> TokenStream {
    input.into_iter().map(|token| {
        match token {
            TokenTree::Group(group) => {
                let stream = replace_for_await(group.stream());
                Group::new(group.delimiter(), stream).into()
            }
            _ => token
        }
    }).collect()
}

#[proc_macro]
pub fn crash(input: TokenStream) -> TokenStream {
    let inner = replace_for_await(input);
    let syn::Block { mut stmts, .. } = syn::parse(inner).unwrap();

    for mut stmt in &mut stmts[..] {
        Dummy.visit_stmt_mut(&mut stmt);
    }   

    quote!(
            let __yield_tx = crate::Sender::new();
            macro_rules! stream_0 {
                () => {{
                    #(#stmts)*
                }}
            }
    )   
    .into()
}
EOF

cargo +nightly run
