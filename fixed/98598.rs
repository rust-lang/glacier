pub trait Foo {
    type Output: Foo;
    
    fn baz() -> Self::Output;
}

pub struct Bar;

impl Foo for &Bar {
    type Output = Bar;

    fn baz() -> Self::Output {
        Foo::baz();

        Self::Output {}
    }
}

pub fn main() {}
