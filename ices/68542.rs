struct Bug {
    a: [(); (|| { 0 })()]
}

fn main() {}
