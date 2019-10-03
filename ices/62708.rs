pub struct MultiDriver<D: ?Sized> {
    d: [std::marker::PhantomData<D>; MultiDriver::<D>::MAX_DRIVERS],
}

impl<D: ?Sized> MultiDriver<D> {
    const MAX_DRIVERS: usize = 10;
}
