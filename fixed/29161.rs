mod a {
    struct A;

    impl Default for A {
        pub fn default() -> A {
            A;
        }
    }
}


fn main() {
    a::A::default();
}
