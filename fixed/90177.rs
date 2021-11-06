trait Base<'f> {
    type Assoc;

    fn do_something(&self);
}

trait ForAnyLifetime: for<'f> Base<'f> {}

impl<T> ForAnyLifetime for T where T: for<'f> Base<'f> {}

trait CanBeDynamic: ForAnyLifetime + for<'f> Base<'f, Assoc = ()> {}

fn foo(a: &dyn CanBeDynamic) {
    a.do_something();
}

fn main() {}
