struct Bad<const N: usize, T> {
    arr: [u8; { N }],
    another: T,
}

fn main() {}
