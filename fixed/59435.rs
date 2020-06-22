#![feature(specialization)]

struct MyStruct {}

trait MyTrait {
    type MyType: Default;
}

impl MyTrait for i32 {
    default type MyType = MyStruct;
}

fn main() {
    let _x: <i32 as MyTrait>::MyType = <i32 as MyTrait>::MyType::default();
}
