pub fn ice(
    x: impl AsRef<str>,
) -> impl IntoIterator<Item = ()> {
    vec![].append(&mut ice(x.as_ref()));

    Vec::new()
}

fn main() {
}
