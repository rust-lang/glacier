macro_rules! some_macro {
    ($other: expr) => ({
        $other(None)
    })
}

fn some_function() {
}

fn main() {
    some_macro!(some_function);
}
