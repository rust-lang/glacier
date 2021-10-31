#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct ConstAssert<const COND: bool>;
trait True {}
impl True for ConstAssert<true> {}

struct Range<T: PartialOrd, const MIN: T, const MAX: T>(T)
where
    ConstAssert<{ MIN <= MAX }>: True;
