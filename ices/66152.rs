fn main() {}

const fn foo<T>() -> usize { 0 }

struct S<'a> {
    beta: [(); foo::<&'a ()>()],
}
