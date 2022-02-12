trait Hrtb<'a> {
    type Assoc;
}

impl<'a> Hrtb<'a> for () {
    type Assoc = ();
}

fn make_impl() -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {}

pub fn main() {}
