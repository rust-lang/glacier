struct S;
impl S {
    fn func<'a, U>(self) -> U {
        todo!()
    }
}
fn dont_crash<'a, U>() {
    S.func::<'a, dont_crash>()
}

fn main() {}
