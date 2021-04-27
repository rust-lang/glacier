#![unstable(feature = "humans", issue = "none")]
#![feature(staged_api)]
struct Foo;
impl Foo {
    #[stable(feature = "rust1", since = "1.0.0")]
    const fn gated() -> u32 {
        42
    }
}

fn main() {}
