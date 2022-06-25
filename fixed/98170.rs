pub struct MyStruct<'a> {
    field: &'a [u32],
}

impl MyStruct<'_> {
    pub fn new<'a>(field: &'a[u32]) -> MyStruct<'a> {
    Self{field}
    }
}

fn main() {}
