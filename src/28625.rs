trait Bar {
    type Bar;
}

struct ArrayPeano<T: Bar> {
    data: T::Bar,
}

fn foo<T>(a: &ArrayPeano<T>) -> &[T] where T: Bar {
    unsafe { std::mem::transmute(a) }
}

impl Bar for () {
    type Bar = ();
}

fn main() {
    let x: ArrayPeano<()> = ArrayPeano { data: () };
    foo(&x);
}
