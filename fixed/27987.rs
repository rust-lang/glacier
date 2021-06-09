pub struct S;
impl S {
    pub fn f<'a>(&self, f: &'a f32) where u32: PartialEq<&'a f32> {
        4==f;
    }
}
fn main() {}
