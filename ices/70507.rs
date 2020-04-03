#![feature(const_generics)]

struct R;
impl R {
    fn method<const N: u8>(&self) {}
}

fn main() {
    R.method::<1u8>();
}
