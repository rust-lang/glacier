trait Tr<T> {
    type Output;
}

impl<T> Tr<T> for () {
    type Output = T;
}

fn g() -> impl for<'a> Tr<&'a u8, Output = impl std::fmt::Debug + 'a> { }

pub fn main() {}
