fn run<'a, A>(x: A)
where
    A: 'static,
    &'static A: ,
{
    let _: &'a A = &x;
}

fn main() {}
