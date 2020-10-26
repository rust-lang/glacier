fn bug<T>() -> impl Iterator<Item = [(); { |x: [u8]| { x } }]> {
    std::iter::empty()
}
