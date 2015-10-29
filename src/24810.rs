use std::ops::Add;

trait T: Add<Output=Self> {
}

impl T<Output=T> {
}

fn main() {
}
