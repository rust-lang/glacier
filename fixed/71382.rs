#![feature(const_generics, const_compare_raw_pointers)]
#![allow(incomplete_features)]

struct Test();

fn pass() {
    ()
}

impl Test {
    pub fn call_me(&self) {
        self.test::<pass>();
    }

    fn test<const FN: fn()>(&self) {
        FN();
    }
}

fn main() {
    let x = Test();
    x.call_me()
}
