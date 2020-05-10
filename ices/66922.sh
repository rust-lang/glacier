#!/bin/bash

# git clone https://github.com/95th/proc-macro-workshop.git && cd proc-macro-workshop

# git checkout ICE

mkdir proc-macro-workshop && cd proc-macro-workshop

cat > Cargo.toml <<EOF
[package]
name = "proc-macro-workshop"
version = "0.0.0"
edition = "2018"
publish = false

[workspace]

[[bin]]
name = "workshop"
path = "main.rs"

[dependencies]
bitfield = { path = "bitfield" }
derive_builder = { path = "builder" }
derive_debug = { path = "debug" }
seq = { path = "seq" }
sorted = { path = "sorted" }
EOF

cat > main.rs <<EOF
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let builder = Command::builder();

    let _ = builder;
}
EOF

mkdir -p bitfield/impl/src bitfield/src

cat > bitfield/impl/src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}
EOF

cat > bitfield/impl/Cargo.toml <<EOF
[package]
name = "bitfield-impl"
version = "0.0.0"
edition = "2018"
publish = false

[lib]
proc-macro = true

[dependencies]
# TODO
EOF

echo "pub use bitfield_impl::bitfield;" > bitfield/src/lib.rs

cat > bitfield/Cargo.toml <<EOF
[package]
name = "bitfield"
version = "0.0.0"
edition = "2018"
autotests = false
publish = false

# [[test]]
# name = "tests"
# path = "tests/progress.rs"

[dev-dependencies]
trybuild = "1.0"

[dependencies]
bitfield-impl = { path = "impl" }
EOF

mkdir -p builder/src

cat > builder/src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let builder_name = format!("{}Builder", ident);
    let builder_ident = syn::Ident::new(&builder_name, ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let opt_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if get_inner_ty("Option", ty).is_some() {
            quote! { #name: #ty }
        } else if builder_of(f).is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: std::option::Option<#ty> }
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let (arg_ty, value) = if let Some(inner_ty) = get_inner_ty("Option", ty) {
            (inner_ty, quote! { std::option::Option::Some(#name) })
        } else if builder_of(f).is_some() {
            (ty, quote! { #name })
        } else {
            (ty, quote! { std::option::Option::Some(#name) })
        };

        let set_method = quote! {
            pub fn #name(&mut self, #name: #arg_ty) -> &mut Self {
                self.#name = #value;
                self
            }
        };

        match get_each_method(f) {
            None => set_method.into(),
            Some((true, each_method)) => each_method,
            Some((false, each_method)) => {
                let methods = quote! {
                    #set_method
                    #each_method
                };
                methods.into()
            }
        }
    });

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if builder_of(f).is_some() {
            quote! { #name: std::vec::Vec::new() }
        } else {
            quote! { #name: std::option::Option::None }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if get_inner_ty("Option", &f.ty).is_some() || builder_of(f).is_some() {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#opt_fields,)*
        }

        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_fields,)*
                }
            }
        }

        impl #builder_ident {
            #(#methods)*

            pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
                std::result::Result::Ok(#ident {
                    #(#build_fields,)*
                })
            }
        }
    };

    expanded.into()
}

fn builder_of(f: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &f.attrs {
        if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
            return Some(attr);
        }
    }
    None
}

macro_rules! err {
    ($meta: expr) => {
        syn::Error::new_spanned($meta, r#"expected `builder(each = "...")`"#).to_compile_error()
    };
}

fn get_each_method(f: &syn::Field) -> Option<(bool, proc_macro2::TokenStream)> {
    let name = &f.ident;
    let ty = &f.ty;

    let attr = builder_of(f)?;

    let meta = match attr.parse_meta() {
        Ok(syn::Meta::List(list)) => list,
        Ok(meta) => return Some((false, err!(meta))),
        Err(e) => return Some((false, e.to_compile_error())),
    };

    let nv = match meta.nested.first() {
        Some(syn::NestedMeta::Meta(syn::Meta::NameValue(nv))) => nv,
        _ => {
            return Some((false, err!(meta)));
        }
    };

    if nv.path.get_ident().unwrap() != "each" {
        return Some((false, err!(meta)));
    }

    match &nv.lit {
        syn::Lit::Str(s) => {
            let ident = syn::Ident::new(&s.value(), s.span());
            let inner_ty = get_inner_ty("Vec", ty).unwrap();
            let method = quote! {
                pub fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#name.push(#ident);
                    self
                }
            };
            Some((Some(ident) == f.ident, method))
        }
        _ => Some((false, err!(meta))),
    }
}

fn get_inner_ty<'a>(outer: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != outer {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(inner_ty) = &p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            for arg in &inner_ty.args {
                return if let syn::GenericArgument::Type(t) = arg {
                    Some(t)
                } else {
                    None
                };
            }
        }
    }
    None
}
EOF

cat > builder/Cargo.toml <<EOF
[package]
name = "derive_builder"
version = "0.0.0"
edition = "2018"
autotests = false
publish = false

[lib]
proc-macro = true

# [[test]]
# name = "tests"
# path = "tests/progress.rs"

[dev-dependencies]
trybuild = "1.0"

[dependencies]
syn = { version = "1.0.8", features = ["extra-traits"] }
quote = "1.0.2"
proc-macro2 = "1.0.6"
EOF

mkdir -p debug/src

cat > debug/src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

macro_rules! err {
    ($meta: expr) => {
        syn::Error::new_spanned($meta, r#"expected `debug = "..."`"#).to_compile_error()
    };
}

fn is_phantom_data_of<'a>(ty: &'a syn::Type, ty_param: &'a syn::TypeParam) -> bool {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        if segments.len() != 1 || segments[0].ident != "PhantomData" {
            return false;
        }

        if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            args,
            ..
        }) = &segments[0].arguments
        {
            if args.len() != 1 {
                return false;
            }

            if let syn::GenericArgument::Type(syn::Type::Path(ty_path)) = &args[0] {
                let path = &ty_path.path.segments[0];
                return path.ident == ty_param.ident;
            }
        }
    }
    false
}

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        unimplemented!();
    };

    let generic_params = ast.generics.type_params();

    let generic_bounds = ast.generics.type_params().map(|ty| {
        for field in fields {
            if is_phantom_data_of(&field.ty, ty) {
                return quote! { #ty };
            }
        }
        quote! { #ty: std::fmt::Debug }
    });

    let fields = fields.iter().map(|f| {
        let ident = &f.ident;
        for attr in &f.attrs {
            let nv = match attr.parse_meta() {
                Ok(syn::Meta::NameValue(nv)) => nv,
                Ok(meta) => return err!(meta),
                Err(e) => return e.to_compile_error(),
            };
            match &nv.lit {
                syn::Lit::Str(s) => {
                    let format = &s.value();
                    return quote! {
                        .field(stringify!(#ident), &format_args!(#format, &self.#ident))
                    };
                }
                _ => return err!(nv),
            }
        }
        quote! {
            .field(stringify!(#ident), &self.#ident)
        }
    });

    let expr = quote! {
        impl<#(#generic_bounds,)*> std::fmt::Debug for #ident<#(#generic_params,)*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                #(#fields)*
                .finish()
            }
        }
    };
    expr.into()
}
EOF

cat > debug/Cargo.toml <<EOF
[package]
name = "derive_debug"
version = "0.0.0"
edition = "2018"
autotests = false
publish = false

[lib]
proc-macro = true

# [[test]]
# name = "tests"
# path = "tests/progress.rs"

[dev-dependencies]
trybuild = "1.0"

[dependencies]
quote = "1.0.2"
syn = "1.0.9"
EOF

mkdir -p seq/seq-macro/src seq/src

cat > seq/seq-macro/src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Literal, TokenStream as TokenStream2, TokenTree};
use proc_macro_hack::proc_macro_hack;
use std::iter::Peekable;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

#[derive(Debug)]
struct Seq {
    ident: syn::Ident,
    start: isize,
    end: isize,
    inclusive: bool,
    body: TokenStream2,
}

impl Parse for Seq {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start = input.parse::<syn::LitInt>()?.base10_parse()?;
        let mut inclusive = true;
        if input.parse::<Token![..=]>().is_err() {
            input.parse::<Token![..]>()?;
            inclusive = false;
        }
        let end = input.parse::<syn::LitInt>()?.base10_parse()?;
        let body;
        syn::braced!(body in input);
        let body = body.parse()?;
        Ok(Self {
            ident,
            start,
            end,
            inclusive,
            body,
        })
    }
}

struct SeqIter {
    end: isize,
    curr: isize,
}

impl SeqIter {
    fn new(seq: &Seq) -> Self {
        let curr = seq.start;
        let end = if seq.inclusive { seq.end + 1 } else { seq.end };
        Self { end, curr }
    }
}

impl Iterator for SeqIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.end {
            None
        } else {
            let out = Some(self.curr);
            self.curr += 1;
            out
        }
    }
}

impl Into<TokenStream> for Seq {
    fn into(self) -> TokenStream {
        let (inner, changed) = self.inner_pass();
        if changed {
            inner.into()
        } else {
            self.outer_pass(inner).into()
        }
    }
}

impl Seq {
    fn iter(&self) -> SeqIter {
        SeqIter::new(self)
    }

    fn expand_group<I>(
        &self,
        tt: TokenTree,
        iter: &mut Peekable<I>,
        out: &mut Vec<TokenTree>,
    ) -> bool
    where
        I: Iterator<Item = TokenTree>,
    {
        match tt {
            TokenTree::Punct(c) if c.as_char() == '#' => match iter.peek() {
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                    let (tts, changed) = self.try_expand_inner(g.stream());
                    let new_g = Group::new(Delimiter::None, tts);
                    out.push(TokenTree::Group(new_g));
                    iter.next();
                    if let Some(TokenTree::Punct(c)) = iter.next() {
                        assert_eq!(c.as_char(), '*');
                    } else {
                        panic!("Expected repetition");
                    }
                    return changed;
                }
                _ => out.extend(vec![TokenTree::Punct(c)]),
            },
            TokenTree::Group(g) => {
                let (tts, changed) = self.try_expand_inner(g.stream());
                let new_g = Group::new(g.delimiter(), tts);
                out.push(TokenTree::Group(new_g));
                return changed;
            }
            tt => out.push(tt),
        }
        false
    }

    fn inner_pass(&self) -> (TokenStream2, bool) {
        self.try_expand_inner(self.body.clone())
    }

    fn try_expand_inner(&self, tts: TokenStream2) -> (TokenStream2, bool) {
        let mut changed = false;
        let mut out = Vec::new();
        let mut iter = tts.into_iter().peekable();
        while let Some(tt) = iter.next() {
            if self.expand_group(tt, &mut iter, &mut out) {
                changed = true;
            }
        }
        (out.into_iter().collect(), changed)
    }

    fn outer_pass(&self, tts: TokenStream2) -> TokenStream2 {
        let mut out = TokenStream2::new();
        for n in self.iter() {
            out.extend(self.expand(tts.clone(), n));
        }
        out
    }

    fn expand(&self, tts: TokenStream2, n: isize) -> TokenStream2 {
        let mut out = Vec::new();
        let mut iter = tts.into_iter().peekable();
        while let Some(tt) = iter.next() {
            match tt {
                TokenTree::Group(g) => {
                    let stream = self.expand(g.stream(), n);
                    let mut group = Group::new(g.delimiter(), stream);
                    group.set_span(g.span());
                    out.push(TokenTree::Group(group));
                }
                TokenTree::Ident(ident) => {
                    if ident == self.ident {
                        let mut lit = Literal::isize_unsuffixed(n);
                        lit.set_span(ident.span());
                        out.push(TokenTree::Literal(lit));
                        continue;
                    }

                    if let Some(TokenTree::Punct(next)) = iter.peek() {
                        if next.as_char() == '#' {
                            iter.next();
                            if let Some(TokenTree::Ident(next)) = iter.peek() {
                                if next == &self.ident {
                                    iter.next();
                                    let mut new_name = format!("{}{}", ident, n);
                                    if let Some(TokenTree::Punct(next)) = iter.peek() {
                                        if next.as_char() == '#' {
                                            iter.next();
                                            if let Some(TokenTree::Ident(suffix)) = iter.next() {
                                                new_name = format!("{}{}{}", ident, n, suffix);
                                            }
                                        }
                                    }
                                    let new_ident = Ident::new(&new_name, ident.span());
                                    out.push(TokenTree::Ident(new_ident));
                                    continue;
                                }
                            }
                        }
                    }
                    out.push(TokenTree::Ident(ident));
                }
                tt => out.push(tt),
            }
        }
        out.into_iter().collect()
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let seq = syn::parse_macro_input!(input as Seq);
    seq.into()
}

#[proc_macro_hack]
pub fn eseq(input: TokenStream) -> TokenStream {
    seq(input)
}
EOF

cat > seq/src/lib.rs <<EOF
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use seq_macro::eseq;

pub use seq_macro::seq;
EOF

cat > seq/Cargo.toml <<EOF
[package]
name = "seq"
version = "0.0.0"
edition = "2018"
autotests = false
publish = false

# [[test]]
# name = "tests"
# path = "tests/progress.rs"

[dev-dependencies]
trybuild = "1.0"

[dependencies]
quote = "1.0.2"
syn = { version = "1.0.9", features = ["full"] }
proc-macro2 = "1.0.6"
proc-macro-hack = "0.5.11"
seq-macro = { path = "./seq-macro" }
EOF

mkdir -p sorted/src

cat > sorted/src/lib.rs <<EOF
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let _ = input;

    unimplemented!()
}
EOF

cat > sorted/Cargo.toml <<EOF
[package]
name = "sorted"
version = "0.0.0"
edition = "2018"
autotests = false
publish = false

[lib]
proc-macro = true

# [[test]]
# name = "tests"
# path = "tests/progress.rs"

[dev-dependencies]
trybuild = "1.0"

[dependencies]
# TODO
EOF

cargo +nightly run
