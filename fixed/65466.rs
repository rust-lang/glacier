#![deny(indirect_structural_match)]

#[derive(PartialEq, Eq)]
enum O<T> {
    Some(*const T), // Can also use PhantomData<T>
    None,
}

struct B;

const C: &[O<B>] = &[O::None];

pub fn foo() {
    let x = O::None;
    match &[x][..] {
        C => (),
        _ => (),
    }
}

fn main() {}
