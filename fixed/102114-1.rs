trait A{type B<'b>;}
struct C;
impl A for C {
    type B<b> =impl;
    fn a() -> Self::B<'a>;
}
