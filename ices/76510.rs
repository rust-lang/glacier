struct Dummy;

impl Dummy {
    const fn func(&mut self) -> usize {
        42
    }
}

const _: &[usize] = &[0; {
    const DUMMY: &Dummy = &Dummy;
    DUMMY.func()
}];
