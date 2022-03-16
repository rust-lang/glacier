struct ErrorKind;
struct Error(ErrorKind);
impl Fn(&isize) for Error {
    fn foo<const N: usize>(&self) -> Self::B<{N}>;
}
fn main() {}
