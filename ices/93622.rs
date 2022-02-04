#![feature(unsized_fn_params)]

pub enum Tree {
    Node(Box<Tree>, Box<Tree>),
    Leaf,
}

pub type Cont<'a> = dyn Fn(u64) -> u64 + 'a;

pub fn height_cps<'a>(x : &'a Tree, k : Cont<'a>) -> u64 {
    match x {
        Tree::Leaf => k(0),
        Tree::Node(l, r) => {
            //                                          v-- causes crash
            height_cps(l, {let k : Box<Cont> = Box::new(move |lh| {
                height_cps(r, {let k : Box<Cont> = Box::new(|lr| { k(1 + lh.max(lr)) }); *k})
            }); *k})
        }
    }
}

fn main () {
    height_cps(&Tree::Leaf, *(Box::new(|i| {i}) as Box<Cont>));
}
