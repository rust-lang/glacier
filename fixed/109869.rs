type Spanned<T> = (T, ());

trait Span<T> {}

impl<T> Span<T> for (T, ()) {}

impl<F, T: From<F>> From<Spanned<F>> for dyn Span<T>
where
    Self: Sized
{
    fn from((from, ()): Spanned<F>) -> Self {
        (T::from(from), ())
    }
}

fn main() {}