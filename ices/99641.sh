#!/bin/bash

rustc -Cdebuginfo=1 - <<'EOF'

#![feature(adt_const_params)]

fn main() {
    pub struct Color<const WHITE: (fn(),)>;

    impl<const WHITE: (fn(),)> Color<WHITE> {
        /// Construct a new color
        pub fn new() -> Self {
            Color::<WHITE>
        }
    }

    pub const D65: (fn(),) = (|| {},);

    Color::<D65>::new();
}

EOF

