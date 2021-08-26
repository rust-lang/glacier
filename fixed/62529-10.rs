trait Mirror { type Image; }
impl<T> Mirror for T { type Image = T; }

fn test<L,T>(l: L) where L: FnOnce(Option<<&T as Mirror>::Image>),
                         for<'a> &'a T: Mirror
{ l(None); }
fn main() {
    test::<_,u8>(|_| {});
}
