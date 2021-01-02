#![crate_type = "lib"]

enum ConstGenericEnum<const N: usize> {
    Foo([i32; N]),
    Bar,
}

fn foo<const N: usize>(val: &ConstGenericEnum<N>) {
    if let ConstGenericEnum::<N>::Foo(field, ..) = val {
        todo!()
    } else {
        todo!()
    }
}
