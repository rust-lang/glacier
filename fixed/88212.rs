#![feature(unsized_locals)]

trait Example {}

struct Foo();

impl Example for Foo {}

fn example() -> Box<dyn Example> {
    Box::new(Foo())
}

fn main() {
    let x: dyn Example = *example();
    (move || {  // ERROR
        let _y = x;
    })();
}
