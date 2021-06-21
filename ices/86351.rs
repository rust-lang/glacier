#![crate_type = "lib"]
pub trait TestTrait {
    type MyType;
    fn func() -> Option<Self> where Self: Sized;
}

impl<T> dyn TestTrait<MyType = T> where Self: Sized {
    fn other_func() -> Option<Self> {
        match Self::func() {
            Some(me) => Some(me),
            None => None
        }
    }
}
