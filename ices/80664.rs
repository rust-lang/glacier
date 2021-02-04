mod inner {
    pub struct Public;
}

pub use inner::Public as Reexported;

fn main() {}
