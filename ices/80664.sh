rustc --output-format json << EOF
mod inner {
    pub struct Public;
}

pub use inner::Public as Reexported;

fn main() {}
EOF
