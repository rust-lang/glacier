rustc --emit=mir -Zmir-opt-level=3 - 2>&1 << EOF

use std::mem;

#[derive(Copy, Clone)]
enum Never {}

union Foo {
    a: u64,
    b: Never
}

fn foo(xs: [(Never, u32); 1]) -> u32 { xs[0].1 }

fn bar([(_, x)]: [(Never, u32); 1]) -> u32 { x }

fn main() {
    println!("{}", mem::size_of::<Foo>());

    let f = [Foo { a: 42 }, Foo { a: 10 }];
    println!("{:?}", unsafe { f[0].a });
}

EOF
