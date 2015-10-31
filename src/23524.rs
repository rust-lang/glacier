enum Tree<K,V> {
    Leaf(K, V),
    Branch(u16, [Option<Box<Tree<K,V>>>]),
}

struct Node<T: ?Sized> {
    population: u16,
    children: T,
}

enum Tree2<K,V> {
    Leaf(K, V),
    Branch(Node<Tree<K,V>>),
}

fn main() {
    use std::mem::size_of;
    use std::sync::atomic;
    println!("{:?}", size_of::<&Tree<(),()>>());
    println!("{:?}", size_of::<&Tree2<(),()>>());
}
