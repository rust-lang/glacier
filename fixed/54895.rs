trait Trait<'a> {
    type Out;
}
impl<'a> Trait<'a> for () {
    type Out = ();
}
fn main() -> impl for<'a> Trait<'a, Out = impl ?Sized + 'a> {
    ()
}
