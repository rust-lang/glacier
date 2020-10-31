#![feature(type_alias_impl_trait)]
#![no_std]

pub trait Tr<'x> {
    type Fut: core::future::Future<Output = ()>;

    fn f() -> Self::Fut;
}

impl<'x> Tr<'x> for () {
    type Fut = impl core::future::Future<Output = ()>;

    fn f() -> Self::Fut {
        async {
            //if false {
            return ();
            //}
            let res: Undef = ();
            res
        }
    }
}
