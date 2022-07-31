#![feature(generic_associated_types)]

struct Node<T, const N : usize, const P : usize> {
    data : [[T; N]; P]
}

trait NodeT<const N : usize> {
    type Node<T>;
}

struct NodeLookup;

macro_rules! impl_lookup {
    ($($n:literal)*) => { $(
        impl NodeT<$n> for NodeLookup {
            type Node<T> = Node<T, $n, {$n*$n}>;
        }
    )* }
}

impl_lookup!(1 2 3 4);

fn main() {}
