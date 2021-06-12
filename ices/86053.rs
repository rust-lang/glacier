trait H<T> {}

unsafe extern "C" fn ordering4<'a, F: H<&'static &'a ()>>(_: (), ...) {}

fn main() {}
