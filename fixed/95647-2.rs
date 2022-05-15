trait Tr<'a> {
    type Assoc;
}

impl<'a> Tr<'a> for () {
    type Assoc = &'a ();
}

fn g() -> impl for<'a> Tr<'a, Assoc = impl Send + 'a> {}
