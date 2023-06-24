enum Either {
    One(X),
    Two(X),
}

struct X;

fn move_into_fnmut() {
    let x = Either::One(X);
    let y = || {
        let Either::Two(a) = x;
    };
}

fn main() { }