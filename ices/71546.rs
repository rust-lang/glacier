pub trait T {
    fn t<F: Fn()>(&self, _: F) {}
}

pub fn crash<V>(v: &V)
where
    for<'a> &'a V: T + 'static,
{
    v.t(|| {});
}

fn main() { }
