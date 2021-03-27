#!/bin/sh

cat > out.rs << EOF
mod inner {
    pub struct Public;
}

pub use inner::Public as Reexported;
EOF

rustdoc out.rs --output-format json
