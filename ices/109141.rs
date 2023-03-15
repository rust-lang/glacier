#![feature(generic_const_exprs)]

impl EntriesBuffer {
    fn a(&self) -> impl Iterator {
        self.0.iter_mut()
    }
}

struct EntriesBuffer(Box<[[u8; HashesEntryLEN]; 5]>);

fn main() {}
