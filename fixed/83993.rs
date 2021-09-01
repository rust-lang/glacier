#![feature(const_generics)]
fn bug<'a>()
where
    for<'b> [(); {
        let x: &'b ();
        0
    }]:
{}
