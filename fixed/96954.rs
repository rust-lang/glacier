trait Tr<'a> {
    type Assoc;
}
fn test_argument_position(x: impl for<'a> Tr<'a, Assoc = impl Copy + 'a>)  {}
fn main() {}
