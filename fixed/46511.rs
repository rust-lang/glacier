struct Foo<'a>
{
    a: [u8; std::mem::size_of::<&'a mut u8>()]
}
