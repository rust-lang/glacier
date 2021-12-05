#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

trait Bar {
    fn bar(&self);
}

trait Foo {
    type FooFn<B>: FnOnce();
    
    fn foo<B: Bar>(&self, bar: B) -> Self::FooFn<B>;
}

struct MyFoo;

impl Foo for MyFoo {
    type FooFn<B> = impl FnOnce();
    
    fn foo<B: Bar>(&self, bar: B) -> Self::FooFn<B> {
        move || { bar.bar() }
    }
}

fn main() {
    let boom: <MyFoo as Foo>::FooFn<u32> = unsafe { core::mem::zeroed() };
    boom();
}
