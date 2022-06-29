struct A<T> {
    inner: T
}

impl <T> A<T>  {
    thread_local! {
        static MY_STATIC: Option<T>  = None;
    }
}

fn main() {}
