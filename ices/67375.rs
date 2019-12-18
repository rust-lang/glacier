struct Bug<T> {
    inner: [(); { [|_: &T| {}; 0].len() }],
}

fn main() {}
