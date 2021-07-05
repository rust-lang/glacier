fn main() {
    struct S<const N: usize>;
    S as *const ();
}
