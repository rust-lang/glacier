#!/bin/bash

rustc -Zunpretty=everybody_loops - <<'EOF'
mod foo {
    pub fn bar() {}
}

fn main() {
    let _ = || -> Result<(), ()> { try!(Ok(())); Ok(()) }; // issue #37345

    macro_rules! m { () => {
        $crate::foo::bar(); // issue #37357
        ::foo::bar(); // issue #38682
    } }
    m!();
}
EOF
