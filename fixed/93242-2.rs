pub fn something(path: &[usize]) -> impl Fn() -> usize {
    || match path {
        [] => 0,
        _ => 1,
    }
}

fn main() {}
