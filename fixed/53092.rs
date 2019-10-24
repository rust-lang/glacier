#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(type_alias_impl_trait)]
#![feature(untagged_unions)]

const fn transmute<T, U>(t: T) -> U {
    union Transform<TT, UU> {
        t: TT,
        u: UU,
    }
    
    unsafe { Transform { t }.u }
}

type Foo<F, U> = impl Fn(F) -> U + Copy;

const BAZR: Foo<u8, ()> = transmute(|x: ()| { x });

fn bar<T, U: From<T>>() -> Foo<T, U> { |x| x.into() }

fn main() {
    let x = BAZR(0);
}
