struct Foo<'a> {
    data: &'a [u8],
}

impl<'a> Foo<'a> {
    const LEN: usize = 4;
    fn bar(buf: &mut [u8; Self::LEN]) {
        unimplemented!()
    }
}
