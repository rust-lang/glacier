trait Trait {
    type Associated;
    fn into(self) -> Self::Associated;
}

impl<'a, I: Iterator<Item = i32>> Trait for (i32, I) {
    type Associated = (i32, impl Iterator<Item = i32>);
    fn into(self) -> Self::Associated { (0_i32, &[0_i32].iter()) }
}

fn main() {}
