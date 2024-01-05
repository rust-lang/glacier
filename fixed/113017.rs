#![feature(generic_const_exprs)]

pub fn String<V>(elem)
where
    V: 'a,
    for<const N: usize = { || {}}> V: 'a, 
    for<C2: , R2, R3: > <&str as IntoIterator>::Item: 'static,
{}

