trait ColumnOutput<'a> {
    type Output;
}

struct C {}

impl<'a> ColumnOutput<'a> for C {
    type Output = u64;
}

fn calc<'a, O, F>(f: F)
where
    O: ColumnOutput<'a>,
    F: Fn(<O as ColumnOutput>::Output) -> u64,
{
    f(vec![].pop().unwrap());
}

fn main() {
    calc::<C, _>(|_| unimplemented!());
}
