#!/bin/bash

cat > out.rs <<'EOF'


mod m1 {
    pub fn f() {}
}
mod m2 {
    pub fn f(_: u8) {}
}

pub use m1::*;
pub use m2::*;

pub mod glob {
    pub use *;
}


EOF

rustdoc --edition=2015 -Zunstable-options --output-format json out.rs
