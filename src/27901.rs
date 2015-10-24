trait Stream { type Item; }
impl<'a> Stream for &'a str { type Item = char; }
fn f<'s>(_: &'s str) -> (&'s str, <&'s str as Stream>::Item) { loop {}}
fn main() {
    f as for<'t> fn(&'t str) -> (&'t str, <&'t str as Stream>::Item);
}
