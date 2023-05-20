#![crate_type = "lib"]
extern crate f;

pub use inner::f;

/// [mod@std::env] [g]
pub use f as g;


fn main() {}