#![feature(generators)]

fn foo() {
    let _y = static || {
        let x = &mut 0;
        *{
            yield;
            x
        } += match { *"" }.len() {
            _ => 0,
        };
    };
}

fn main() {
    foo()
}
