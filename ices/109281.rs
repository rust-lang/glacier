#![feature(type_alias_impl_trait)]
struct S;
type ReturnType<'a> = impl Eq + 'a;
impl std::ops::Deref for S {
    type Target = dyn Fn(&()) -> ReturnType;
    fn deref() -> &'static Self::Target {}
}

fn main() {}
