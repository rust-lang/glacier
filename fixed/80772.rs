trait SomeTrait {}

struct Exhibit {
    constant: usize,
    factory: fn(&usize) -> Box<dyn SomeTrait>,
}

const A_CONSTANT: &[Exhibit] = &[
    Exhibit {
        constant: 1,
        factory: |_| unimplemented!(),
    },
    Exhibit {
        constant: "Hello world".len(),
        factory: |_| unimplemented!(),
    },
];

fn main() {}
