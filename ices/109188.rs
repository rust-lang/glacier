enum Either {
    One(X),
    Two(X),
}

struct X(Y);

struct Y;

fn move_into_fnmut() {
    let x = move_into_fnmut();

    consume_fnmut(|| {
        X(_t) = x;

        Either::Two(_t) = x;
    });
}

fn main() {}
