rustc --edition 2021 - 2>&1 << EOF

pub fn something(path: &[usize]) -> impl Fn() -> usize {
    || match path {
        [] => 0,
        _ => 1,
    }
}

pub fn main() {}

EOF
