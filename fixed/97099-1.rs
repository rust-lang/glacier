pub trait Trait<'a> {
    type Assoc;
}

struct Struct;
impl<'a> Trait<'a> for Struct {
    type Assoc = &'a u32;
}

fn blah() -> impl for<'a> Trait<'a, Assoc = impl Sized> {
    Struct
}
