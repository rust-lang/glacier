fn foo<'a: 'a>() -> impl Sized + 'a {
    let _: *mut &'a () = foo::<'a>();
    loop {}
}

fn main() {}
