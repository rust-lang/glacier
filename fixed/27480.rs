pub enum FragmentRepr {
    Boxed(Box<FragmentRepr>),
    Enum(()),
}
fn unconstrained<T>() -> T { loop {} }
pub fn foo(mut entry: &mut FragmentRepr) {
    entry = if let &mut FragmentRepr::Boxed(ref mut contents) = entry {
        contents
    } else {
        unconstrained()
    };
    match *entry {
        FragmentRepr::Enum(ref mut discrs) => {},
        _ => {}
    };
}
fn main() {}
