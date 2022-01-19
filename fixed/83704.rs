use std::ops::Deref;

struct VecWrapper<T, const N: usize>(Vec<T>);

impl<T, const N: usize> VecWrapper<T, N> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<T, const N: usize> Deref for VecWrapper<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

fn foo() -> VecWrapper<i32, 10> {
    let tv = VecWrapper::new();
    let _ = tv.len(); // required to trigger ICE?
    tv
}
