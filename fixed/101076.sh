#!/bin/bash

cat > out.rs <<'EOF'

const _: () = {
    #[macro_export]
    macro_rules! first_macro {
        () => {}
    }
    mod foo {
        #[macro_export]
        macro_rules! second_macro {
            () => {}
        }
    }
};

EOF

rustdoc out.rs
