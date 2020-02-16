#![feature(impl_trait_in_bindings)]

fn bug<'a, 'b, T>()
where
    'a: 'b,
{
    let f: &impl Fn(&'a T) -> &'b T = &|x| x;
}

fn main() {}
