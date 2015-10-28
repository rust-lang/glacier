macro_rules! foo {
    ($y:expr) => ({
        $y = 2;
    })
}

fn main() {
    let mut x = 1;
    foo!(x);
}
