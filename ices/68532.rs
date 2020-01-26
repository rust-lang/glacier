struct A<'a>(&'a ());

impl<'a> A<'a> {
    const N: usize = 68;

    fn foo(&self) {
        let mut b = [0; Self::N];
    }
}

fn main() {}
