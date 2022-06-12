struct TestErr<T> {
     field: std::marker::PhantomData<T>,
}

impl<T> TestErr<T> {
    fn func_a() {}

    fn func_b() {
        Self::func_a();

        let variable = None;
    }
}

fn main() {}
