enum Foo {
    NotLast(Box<Foo>),
    Last(u32)
}

fn x(tree: &mut Foo) {
    let mut current = tree;

    loop {
        match current {
            &mut Foo::NotLast(ref mut inner) => {
                current = &mut *inner;
            },
            &mut Foo::Last(ref mut val) => {
                *val = 1;
            },
        }
    }
}

fn main() {}
