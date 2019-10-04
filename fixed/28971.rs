enum Foo {
    Bar(u8)
}
fn main(){
    foo(|| {
        match Foo::Bar(1) {
            Foo::Baz(..) => (), //~ ERROR no associated
            _ => (),
        }
    });
}

fn foo<F>(f: F) where F: FnMut() {
    f();
}
