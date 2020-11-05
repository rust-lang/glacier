macro_rules! bug {
    ($expr:expr) => {
        #[rustc_dummy = $expr] // Any key-value attribute, not necessarily `doc`
        struct S;
    };
}

// Any expressions containing macro call `X` that's more complex than `X` itself.
// Parentheses will work.
bug!((line!()));

fn main() {}
