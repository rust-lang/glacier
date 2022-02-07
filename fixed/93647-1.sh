rustc --edition 2021 - 2>&1 << EOF


struct X<const N: usize = {
    async move {}
}>;

pub fn main() {}

EOF
