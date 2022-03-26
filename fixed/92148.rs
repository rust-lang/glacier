trait Blah<T> {
    type Output;
    fn foo(t: T) -> Self::Output;
}

impl Blah<i32> for i32 {
    type Output = i64;

    fn foo(t: i32) -> Self::Output {
        t as Self::Output
    }
}

impl<B, F> Blah<Vec<F>> for B
where
    B: Blah<F>,
{
    type Output = Vec<T::Output>;

    fn foo(t: T) -> Self::Output {
        t.into_iter(F::foo).collect()
    }
}
pub fn main() {}
