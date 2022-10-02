use std::marker::PhantomData;

pub struct NfaBuilder<'brand> {
    brand: PhantomData<&'brand mut &'brand mut ()>,
}

impl NfaBuilder<'_> {
    pub fn with<R, F: FnOnce(NfaBuilder<'_>) -> R>(f: F) -> R {
        Brand::with(|brand| {
            // This should be using NfaBuilder instead of Self becuase they have diffrent lifetime constraints
            f(Self {
                brand: brand.lt,
            })
        })
    }
}

#[derive(Clone, Copy)]
pub struct Brand<'brand> {
    lt: PhantomData<&'brand mut &'brand mut ()>,
}

impl Brand<'_> {
    pub fn with<R, F: FnOnce(Brand<'_>) -> R>(f: F) -> R {
        f(Self { lt: PhantomData })
    }
}

pub fn main() {}
