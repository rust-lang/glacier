#![feature(generic_const_exprs)]

trait MyTrait<T> {}

fn bug<'a, T>() -> &'static dyn MyTrait<[(); { |x: &'a u32| { x }; 4 }]> {
    todo!()
}
