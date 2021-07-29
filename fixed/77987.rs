#![feature(min_type_alias_impl_trait)]
#![feature(type_alias_impl_trait)]

trait Foo<T> {}
impl<T, U> Foo<T> for U {}

type Scope = impl Foo<()>;

#[allow(unused)]
fn infer_scope() -> Scope {
    ()
}

#[allow(unused)]
fn ice() -> impl Foo<Scope> {
    loop {}
}

fn main() {}
