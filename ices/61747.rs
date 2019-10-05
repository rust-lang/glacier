#![feature(const_generics)]

struct Const<const N: usize>;

impl<const C: usize> Const<{C}> {
    fn successor() -> Const<{C + 1}> {
        Const
    }
}

fn main() {
    Const::<1>::successor();
}
