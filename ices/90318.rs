#![feature(const_type_id)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
use core::intrinsics::type_id;

struct If<const B: bool>;
pub trait True {}
impl True for If<true> {}

fn consume<T: 'static>(_val: T)
where
    If<{ type_id::<T>() != type_id::<()>() }>: True,
{
}

fn test<T: 'static>()
where
    If<{ type_id::<T>() != type_id::<()>() }>: True,
{
}

fn main() {
    let a = ();
    consume(0i32);
    consume(a);
    // uncomenting these lines won't result in ICE
    // test::<i32>();
    // test::<()>();
}
