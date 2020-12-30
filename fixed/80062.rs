fn sof<usize>() -> usize {}
fn test<T>() {
    let _: [u8; sof::<T>()];
}
