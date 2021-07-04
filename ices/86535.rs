#![crate_type = "lib"]
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]                                                                                                                                                                                                                

pub trait Foo {
    const ASSOC_C: usize;
    fn foo() where [(); Self::ASSOC_C]:;
}

struct Bar<const N: &'static ()>;
impl<const N: &'static ()> Foo for Bar<N> {
    const ASSOC_C: usize = 3;

    fn foo() where [u8; Self::ASSOC_C]: {
        let _: [u8; Self::ASSOC_C] = loop {};
    }
}
