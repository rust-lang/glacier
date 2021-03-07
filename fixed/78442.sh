#!/bin/bash

rustc -Z mir-opt-level=3 - << EOF
#![crate_type = "lib"]

// Error won't happen if "bar" is not generic
pub fn bar<P>(_baz: P) {
    hide_foo()();
}

// Error won't happen if "iterate" hasn't impl Trait or has generics
fn hide_foo() -> impl Fn() { 
    foo
}

// Error won't happen if "foo" isn't used in "iterate" or has generics
fn foo() {}

EOF
