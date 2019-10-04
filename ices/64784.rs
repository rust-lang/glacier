#![feature(decl_macro)]

pub macro m($i:ident, $j:ident) {
    mod $i {
        pub use crate::$j::*;
        pub struct A;
    }
}

m!(x, y);
m!(y, x);
