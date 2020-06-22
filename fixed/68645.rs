#![feature(generic_associated_types)]

trait Fun {
    type F<'a>: Fn() -> u32;
    
    fn callme<'a>(f: Self::F<'a>) -> u32 {
        f()
    }
}

impl <T> Fun for T {
    type F<'a> = Self;
}

fn main() {
    <&dyn Iterator<Item = u8>>::callme(&std::iter::once(1));
}
