pub struct B;
pub fn a() -> B {
    B
}

mod handlers {
    pub struct C(B);
    pub fn c() -> impl Fn() -> C {
        let a1 = ();
        || C((crate::a(), a1).into())
    }
}

fn main() {}
