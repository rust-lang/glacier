pub struct Node<NodeRef = Box<Self>>(NodeRef);

pub struct Foo(Node);

fn main() {}
