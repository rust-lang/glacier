#![feature(generic_associated_types)]

trait Fun {
    type F<'a>: ?Sized;
    
    fn identity<'a>(t: &'a Self::F<'a>) -> &'a Self::F<'a> { t }
}

impl <T> Fun for T {
    type F<'a> = Self;
}

fn bug<'a, T: Fun<F = dyn Sync + Send>>(_: T) -> Box<T::F<'a>> {
    Box::new(T::identity(&()))
}


fn main() {
    let x = 10;
    
    bug(x);
}
