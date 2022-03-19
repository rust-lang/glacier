rustc -Cdebuginfo=2 - 2>&1 << EOF

struct S<T> { x: [T; !0] }

pub fn f() -> usize {
    std::mem::size_of::<S<u8>>()
}

fn main() {
    let x = f();
}

EOF
