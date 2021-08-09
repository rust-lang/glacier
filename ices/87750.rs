#![feature(generic_associated_types)]
use std::rc::Rc;
use std::ops::Deref;

trait PointerFamily {
    type Pointer<T>: Deref<Target=T> + Sized;

    fn new<T>(obj: T) -> Self::Pointer<T>;
}

#[derive(Debug)]
struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = Rc<T>;

    fn new<T>(obj: T) -> Rc<T> {
        Rc::new(obj)
    }
}

#[derive(Debug)]
enum Node<T, P: PointerFamily> where P::Pointer<Node<T, P>>: Sized {
    Cons(T, P::Pointer<Node<T, P>>),
    Nil
}

type List<T, P> = <P as PointerFamily>::Pointer<Node<T, P>>;
type RcList<T> = List<T, RcFamily>;
type RcNode<T> = Node<T, RcFamily>;

impl<T, P: PointerFamily> Node<T, P> where P::Pointer<Node<T, P>>: Sized {
    fn new() -> P::Pointer<Self> {
        P::new(Self::Nil)
    }

    fn cons(head: T, tail: P::Pointer<Self>) -> P::Pointer<Self> {
        P::new(Self::Cons(head, tail))
    }
}

fn main() {
    let mut list: RcList<i32> = RcNode::<i32>::new();
    list = RcNode::<i32>::cons(1, list);
    //println!("{:?}", list);

}
