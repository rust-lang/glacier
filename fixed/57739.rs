trait ArraySizeTrait {
     const SIZE: usize = 0;
}

// remove this impl to get error E0599 instead of ICE
impl<T : ?Sized> ArraySizeTrait for T {
    const SIZE: usize = 1;
}

struct SomeArray<T: ArraySizeTrait> {
    array: [u8; T::SIZE],
    phantom: std::marker::PhantomData<T>,
}

fn main() {

}
