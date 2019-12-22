struct Bug {
    inner: [(); match || 1 {
        n => n(),
    }],
}

fn main() {}
