trait Foo<'t> {
    type Inner;
}


fn foo<T>(_f: for<'f> fn(<T as Foo<'f>>::Inner))
    where T: for<'t> Foo<'t>
{ }

impl<'t> Foo<'t> for i32 {
    type Inner = &'t i32;
}

fn main() { foo::<i32>(|_inner| { }); }
