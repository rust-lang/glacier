#![feature(non_lifetime_binders)]

pub fn bar()
where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator
{
}

fn main() {
    bar();
}

