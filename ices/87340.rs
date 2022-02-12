type PartiallyDefined<T> = impl 'static;
fn partially_defined() -> PartiallyDefined<_> {}
