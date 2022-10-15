pub trait Trait {
    type Fut<'a> where Self: 'a;
    fn fun<'a, 'b>(&'a self, x: &'_ mut &'b ()) -> Self::Fut<'a>
    where
        'b: 'a;
}
impl Trait for () {
    type Fut<'a> = impl ::std::future::Future + 'a
    where
        Self: 'a;
    fn fun<'a, 'b>(&'a self, x: &'_ mut &'b ()) -> Self::Fut<'a>
    where
        'b: 'a,
    {
        async { }
    }
}

pub fn main() {}
