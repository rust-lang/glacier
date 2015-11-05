/// A trait that produces a clone of the previously yielded element along
/// with the current element.
pub trait ZipPrev: Iterator + Sized {
    fn zip_prev(self) -> Items<Self::Item, Self> {
        Items {
            maybe_prev: None,
            iter: self,
        }
    }
}

/// The iterator produced by the zip_prev method.
pub struct Items<A, I> {
    maybe_prev: Option<A>,
    iter: I,
}

impl<I> Iterator for Items<I::Item, I>
    where
        I: Iterator,
        I::Item: Clone,
{
    type Item = (I::Item, Option<I::Item>);
    #[inline]
    fn next(&mut self) -> Option<(I::Item, Option<I::Item>)> {
        let Items { ref mut iter, ref mut maybe_prev } = *self;
        if let Some(item) = iter.next() {
            let old_maybe_prev = maybe_prev.take();
            *maybe_prev = Some(item.clone());
            Some((item, old_maybe_prev))
        } else {
            None
        }
    }
}

impl<I> ZipPrev for I
    where
        I: Iterator,
        I::Item: Clone {}

fn main() {}
