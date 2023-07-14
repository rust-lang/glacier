fn foo<'a>() -> impl Sized + 'a {
    let i: i32 = foo();
    i
}

fn main() {}
