impl X {
    fn getn<const N: usize>() -> [u8; N] {
        getn::<N>()
    }
}
fn getn<const N: cfg_attr>() -> [u8; N] {}
