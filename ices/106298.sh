#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "alba"
version = "0.0.1"
edition = "2021"

[dependencies]
sera = { path = "sera/" }
EOF

mkdir -p src sera/src

cat > sera/Cargo.toml <<'EOF'
[package]
name = "sera"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["proc-macro"]
EOF

cat > sera/src/lib.rs << EOF
extern crate proc_macro;
struct PanicOnDrop;

impl Drop for PanicOnDrop {
    fn drop(&mut self) { panic!("panic on drop!"); }
}

#[proc_macro_derive(Panic)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _p_on_d = PanicOnDrop;
    panic!("panic during panic-during-expand's derive")
}
EOF

cat > src/main.rs << EOF
#[macro_use]
extern crate sena;

#[derive(sena::Panic)]
struct S { x: u8 }

fn main() {}
EOF

cargo check
