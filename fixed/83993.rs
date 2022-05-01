#![feature(generic_const_exprs)]
fn bug<'a>()
where
    for<'b> [(); {
        let x: &'b ();
        0
    }]:
{}
