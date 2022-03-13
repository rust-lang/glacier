struct Bug<S> {
    A: [(); {
        let x: [u8; Self::W] = [0; Self::W];
        F
    }
}

fn main() {}
