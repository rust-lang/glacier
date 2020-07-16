#![feature(const_generics, in_band_lifetimes)]
#![allow(incomplete_features)]

use std::mem::MaybeUninit;

trait Foo<'a, A>: Iterator<Item = A> {
    fn bar<const N: usize>(&mut self) -> [A; N];

    fn foo<const N: usize>(&mut self) -> [A; N] {
        let result = self.bar();
        result
    }
}

impl<A, I: ?Sized> Foo<'a, A> for I
where
    I: Iterator<Item = A>,
{
    fn bar<const N: usize>(&mut self) -> [A; N] {
        let mut result: [MaybeUninit<A>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for (a, b) in result.iter_mut().zip(self) {
            *a = MaybeUninit::new(b);
        }
        let spam: *const [MaybeUninit<A>; N] = &result;
        unsafe { std::ptr::read(spam as *const [A; N]) }
    }
}

fn main() {
    let _: [u8; 10] = (0_u8..10).foo::<10_usize>();
}
