struct S {
    d: [u32; {
        #![cfg_attr(not(X),Y) Z]
    }
}

fn main() {}
