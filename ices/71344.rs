pub struct DirEnts<'a> {
    pub cluster: ::std::marker::PhantomData<&'a ()>,
}
impl<'a> DirEnts<'a> {
    pub fn next(&mut self) -> [u8; 12] {
        // NOTE: Tuple destructure needed
        // NOTE: Arithmatic needed
        let (outname,) = ([0u8; 12 + 0],);
        outname
    }
}

fn main() {}
