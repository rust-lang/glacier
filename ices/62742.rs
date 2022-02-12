use std::marker::PhantomData;

fn _alias_check() {
    WrongImpl::foo(0i32);       // crash
    WrongImpl::<()>::foo(0i32); // fine
    CorrectImpl::foo(0i32);     // fine
}

pub trait Raw<T: ?Sized> {
    type Value;
}

pub type WrongImpl<T> = SafeImpl<T, RawImpl<T>>;

pub type CorrectImpl<T> = SafeImpl<[T], RawImpl<T>>;

pub struct RawImpl<T>(PhantomData<T>);

impl<T> Raw<[T]> for RawImpl<T> {
    type Value = T;
}

pub struct SafeImpl<T: ?Sized, A: Raw<T>>(PhantomData<(A, T)>);

impl<T: ?Sized, A: Raw<T>> SafeImpl<T, A> {
    pub fn foo(value: A::Value) {}
}
