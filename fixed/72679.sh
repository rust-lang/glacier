#!/bin/bash

rustc -Z mir-opt-level=2 - << EOF

#![cfg_attr(const_fn, feature(const_fn))]

// Ctor(..) is transformed to Ctor { 0: ... } in HAIR lowering, so directly
// calling constructors doesn't require them to be const.

type ExternalType = std::panic::AssertUnwindSafe<(Option<i32>, Result<i32, bool>)>;

const fn call_external_constructors_in_local_vars() -> ExternalType {
    let f = Some;
    let g = Err;
    let h = std::panic::AssertUnwindSafe;
    let x = f(5);
    let y = g(false);
    let z = h((x, y));
    z
}

const CALL_EXTERNAL_CONSTRUCTORS_IN_LOCAL_VARS: ExternalType = {
    let f = Some;
    let g = Err;
    let h = std::panic::AssertUnwindSafe;
    let x = f(5);
    let y = g(false);
    let z = h((x, y));
    z
};


fn main() {
    assert_eq!(
        (call_external_constructors_in_local_vars().0,),
        (CALL_EXTERNAL_CONSTRUCTORS_IN_LOCAL_VARS.0,)
    );
}

EOF
