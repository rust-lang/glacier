struct S1;
struct S2;

trait Foo<T:?Sized> {
    fn foo_method(&self, &T);
}

trait Bar {
    fn bar_method(&self) -> Self::A;
    type A:Foo<Self>;
}

impl Foo<S1> for S2 {
    fn foo_method(&self, _other:&S1) {
        println!("S1");
    }
}

impl Bar for S1 {
    fn bar_method(&self) -> S2 { S2 }
    type A = S2;
}

fn play<T:?Sized+Bar>(arg:&T) {
    arg.bar_method().foo_method(arg);
}

fn main() {
    play(&S1 as &Bar<A=S2>);
}
