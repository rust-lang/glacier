#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]
#![feature(fn_traits)]

trait MyTrait {}
impl MyTrait for () {}

impl<F> FnOnce<()> for &F {
    type Output = impl MyTrait ;
    fn call_once(self, _:() ) -> Self::Output { }
}

fn main() {}
