use std::sync::mpsc::channel;

pub struct S<T> {
    pub thing: T,
}

#[derive(Debug)]
struct E;

impl std::fmt::Display for E {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for E {}

impl<T> S<T> {
    pub fn do_thing()
    where
        for<'a> &'a T: 'static,
    {
        let (_sender, rx) = channel::<T>();
        let _ = rx.recv().map_err(|_| E);
    }
}

fn main() {}
