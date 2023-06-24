#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "kandis"
version = "0.0.1"
edition = "2021"
EOF

mkdir -p src

cat > src/main.rs <<'EOF'
mod foo_1;

/// Hello [`Bar`]. // crash
pub use foo_1::Bar;
EOF

cat > src/foo_1.rs <<'EOF'
pub trait Bar {}
EOF

cargo doc
