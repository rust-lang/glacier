use std::marker::PhantomData as Boo;

struct Gc<'gc, T: 'gc>(Boo<fn(&'gc T) -> &'gc T>);

trait Rootable<'env> {
    type AsRoot<'r>: Rootable<'r> + 'r
    where
        'env: 'r;
}

impl<'env, T: Rootable<'env>> Rootable<'env> for Gc<'env, T> {
    type AsRoot<'r> = Gc<'r, T::AsRoot<'r>> where 'env: 'r;
}

impl<'env> Rootable<'env> for i32 {
    type AsRoot<'r> = i32 where 'env: 'r;
}

fn reroot<'gc, T: Rootable<'gc>>(_t: T, _f: for<'a> fn(T::AsRoot<'a>)) {}

fn test<'gc>(t: Gc<'gc, i32>) {
    reroot(t, |_| ());
}

fn main() {}
