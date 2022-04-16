struct Bug<A = [(); (let a = (), 1).1]> {
    a: A   
}

fn main() {}
