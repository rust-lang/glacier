struct DataWrapper<'a> {
    data: &'a [u8; Self::SIZE],
}

impl DataWrapper<'_> {
    const SIZE: usize = 14;
}

fn main() {}
