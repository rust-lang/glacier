fn hof<F>(_: F)
where
    F: FnMut() -> (),
{
}

fn f() -> _ {
    hof(f);
}

fn main() {}
