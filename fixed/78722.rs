#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

type F = impl core::future::Future<Output = u8>;

struct Bug {
    V1: [(); {
        fn concrete_use() -> F {
            async {}
        }
        let f: F = async { 1 };
        1
    }],
}

fn main() {}
