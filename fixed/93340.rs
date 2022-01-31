#![feature(generic_associated_types)]

pub trait Scalar: 'static {
    type RefType<'a>: ScalarRef<'a>;
}

pub trait ScalarRef<'a>: 'a {}

impl Scalar for i32 {
    type RefType<'a> = i32;
}

impl Scalar for String {
    type RefType<'a> = &'a str;
}

impl<'a> ScalarRef<'a> for i32 {}

impl<'a> ScalarRef<'a> for &'a str {}

fn cmp_eq<'a, 'b, A: Scalar, B: Scalar>(a: A::RefType<'a>, b: B::RefType<'b>) -> bool {
    todo!()
}

fn str_contains(a: &str, b: &str) -> bool {
    a.contains(b)
}

fn build_expression<A: Scalar, B: Scalar, O: Scalar>(
) -> impl Fn(A::RefType<'_>, B::RefType<'_>) -> O {
    cmp_eq::<i32, i32>
}

fn main() {
    println!("Hello, world!");
}
