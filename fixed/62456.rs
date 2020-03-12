#![feature(const_generics)]

use std::fmt;

struct Builder<const N: usize> {
    items: [&'static str; N],
}

fn new_builder() -> Builder<{0}> {
    return Builder{items: []};
}

impl<const N: usize> Builder<{ N }> {
    fn append(self, value: &'static str) -> Builder<{ N + 1 }> {
        let mut new_items = [""; N + 1];
        new_items[..N].copy_from_slice(self.items);
        new_items[N] = value;
        return Builder { items: new_items };
    }

    fn build(self) -> Final<{ N }> {
        return Final { items: self.items };
    }
}

struct Final<const N: usize> {
    items: [&'static str; N],
}

impl<const N: usize> fmt::Debug for Final<{ N }> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Final")
            .field("items", &&self.items[..])
            .finish()
    }
}

fn main() {
    let f = new_builder()
        .append("abc")
        .append("def")
        .append("ghi")
        .build();
    println!("f={:?}", f);
}
