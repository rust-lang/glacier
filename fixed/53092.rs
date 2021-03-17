#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(type_alias_impl_trait)]
#![feature(untagged_unions)]

const fn transmute<T, U>(from: T) -> U {
    use std::mem::ManuallyDrop;

    union Transform<FROM, INTO> {
        from: ManuallyDrop<FROM>,
        into: ManuallyDrop<INTO>,
    }

    let transformer = Transform {
        from: ManuallyDrop::new(from),
    };

    unsafe { ManuallyDrop::into_inner(transformer.into) }
}

type Bug<T, U> = impl Fn(T) -> U + Copy;

const CONST_BUG: Bug<u8, ()> = transmute(|_: u8| ());

fn make_bug<T, U: From<T>>() -> Bug<T, U> {
    |x| x.into()
}

fn main() {
    let x = CONST_BUG(0);
}
