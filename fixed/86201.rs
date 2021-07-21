#![feature(unboxed_closures)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]

type FunType = impl Fn<()>;
static STATIC_FN: FunType = some_fn;

fn some_fn() {}

fn main() {
    let _: <FunType as FnOnce<()>>::Output = STATIC_FN();
}
