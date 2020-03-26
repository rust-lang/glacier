fn bug() {
    [0; match [|f @ &ref _| () ] {} ]
}

fn main() {}
