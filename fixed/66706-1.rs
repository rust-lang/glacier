fn bug() {
    [0; [|&_: _ &_| {}; 0 ].len()]
}

fn main() {}
