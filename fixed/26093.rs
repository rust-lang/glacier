macro_rules! not_an_lvalue {
    ($thing:expr) => {
        $thing = 42;
    }
}

fn main() {
    not_an_lvalue!(99);
}
