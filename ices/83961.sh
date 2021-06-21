#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "abc"
version = "0.0.1"
edition = "2018"
EOF

mkdir -p src

cat > src/main.rs <<'EOF'
fn main() {
    println!("Hello, world!");
}
EOF

cargo test

cat >> src/main.rs <<'EOF'
/// Examples:
/// ```
/// assert_eq!(fun(), 5);
/// ```
fn fun() -> u8 {
    5
}
EOF

cargo test

cat >> Cargo.toml <<'EOF'

[[bin]]
name = "icebin"
path = "src/bin.rs"

[lib]
name = "icelib"
path = "src/lib.rs"
EOF

cat > src/main.rs <<'EOF'
fn main() {
    println!("Hello, world!");
}
EOF

cat > src/lib.rs <<'EOF'
/// Examples:
/// ```
/// assert_eq!(icelib::fun(), 5);
/// ```
pub fn fun() -> u8 {
    5
}
EOF

mv src/main.rs src/bin.rs

cargo test

mkdir -p src/bin
mv src/bin.rs src/bin/main.rs

sed 's|src/bin.rs|src/bin/main.rs|' Cargo.toml

cargo test
