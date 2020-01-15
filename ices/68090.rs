pub fn main() {
    |_: ARef| ();
}

type ARef<'a> = &'a <() as ArrayLength>::ArrayType;

trait ArrayLength {
    type ArrayType;
}

impl ArrayLength for () {
    type ArrayType = u8;
}
