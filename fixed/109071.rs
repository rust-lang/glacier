struct Windows<T> {}

impl<T> Windows {
    type Item = &[T];

    fn next() -> Option<Self::Item> {}
}

impl<T> Windows<T> {
    fn T() -> Option<Self::Item> {}
}
