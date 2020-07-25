pub trait SIterator {}

pub trait Ty<'a> {
    type V;
}

struct FilterMap<F>(F);

impl<X, F> SIterator for FilterMap<F>
where
    F: FnOnce(<X as Ty<'_>>::V) -> Option<<X as Ty<'_>>::V>
{}

fn main() {}
