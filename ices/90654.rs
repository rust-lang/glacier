#![feature(generic_const_exprs)]

trait TyNum {
    const VAL: usize;
}

fn make_array<T: TyNum>() -> [i32; T::VAL + 1] {
    panic!()
}

trait SmallArray {}
impl SmallArray for [i32; 1] {}
impl SmallArray for [i32; 2] {}

fn consume<T: SmallArray>(t: T) {}

fn foo<T>()
where
    T: TyNum,
    [(); T::VAL + 1]: ,
{
    consume(make_array::<T>());
}
