#![feature(type_alias_impl_trait)]
#![no_std]
#![crate_type = "lib"]

pub trait AssociatedImpl {
    type ImplTrait;

    fn f() -> Self::ImplTrait;
}

trait DynTrait<'a> {}

struct S<T>(T);

trait Associated {
    type A;
}

impl<'a, T: Associated<A = dyn DynTrait<'a>>> AssociatedImpl for S<T> {
    type ImplTrait = impl core::future::Future<Output = ()>;

    fn f() -> Self::ImplTrait {
        async { () }
    }
}
