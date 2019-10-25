#![feature(const_generics)]
struct NoMatch;

fn foo<const T: NoMatch>() -> bool {
    return true
}

fn main() {
    foo::<{NoMatch}>();
}
