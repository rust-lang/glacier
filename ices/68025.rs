// https://github.com/rust-lang/rust/issues/68025
// simplified by Centril
fn foo<F, G>(_: G, _: Box<F>)
    where
        F: Fn(),
        G: Fn(Box<F>),
{
}

fn main() {
    foo(|f| (*f)(), Box::new(|| {}));
}