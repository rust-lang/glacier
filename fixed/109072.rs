trait Lt<'_> {
    type T = ();
}
impl<'f> Lt<'a> for () {
    type T = ();
}

fn main() {
    let v: <() as Lt<'_>>::T = ();
}
