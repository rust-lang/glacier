fn bug() {
    [0; [|f @ &ref _| {} ; 0 ].len() ];
}

fn main() {}
