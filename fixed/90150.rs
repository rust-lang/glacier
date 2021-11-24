#![feature(inline_const)]
#![feature(generic_const_exprs)]

const fn foo(val: usize) -> usize { val }

fn bar<const N: usize>(num: usize) -> &'static str
where [(); foo(N)]:
{
    match num {
        const { foo(N) } => "fizz",
        _ => "buzz"
    }
}

pub fn main() {}
