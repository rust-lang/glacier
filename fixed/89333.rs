fn main() {
    test(&|| 0);
}

fn test<T>(arg: &impl Fn() -> T)
where
    for<'a> &'a T: Default,
{
}
