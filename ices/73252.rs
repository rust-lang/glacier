pub trait Foo {
    type Assoc;
}

extern "C" {
    pub fn lint_me() -> <() as Foo>::Assoc;
}

fn main() {}
