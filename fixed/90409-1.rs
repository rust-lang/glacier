#![feature(type_alias_impl_trait)]

trait Bar {
    fn bar(&self);
}

type FooFn<B> = impl FnOnce();

fn foo<B: Bar>(bar: B) -> FooFn<B> {
    move || { bar.bar() }
}

fn main() {
    let boom: FooFn<u32> = unsafe { core::mem::zeroed() };
    boom();
}
