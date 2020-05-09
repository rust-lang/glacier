const fn foo(n: usize) -> usize { n * 2 }
fn bar<const N: usize>() -> [u32; foo(N)] {
    [0; foo(N)]
}
fn main() {}
