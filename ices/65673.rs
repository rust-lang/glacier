pub struct SharedRange<Ix, OnSet> {
    pub data: f32,
}

trait HasCtx {
    type Ctx;
}

trait Callback0 {}

trait Foo<Ix, OnSet> = where OnSet: Callback0;

impl<Ix, OnSet> HasCtx for SharedRange<Ix, OnSet> {
    type Ctx = Foo<Ix, OnSet>;
}

fn main() {}
