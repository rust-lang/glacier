trait Borrowed<'a> {
    type Type: Default;
}

impl<'a> Borrowed<'a> for i32 {
    type Type = i32;
}

fn g<T, F>(f: F)
where
    T: for<'a> Borrowed<'a>,
    F: Fn(<T as Borrowed>::Type),
{
    f(Default::default())
}

fn main() {
    g::<i32, _>(|_| ());
}
