#![crate_type = "lib"]

pub trait First {
    const CONST: bool;
}
pub trait Second {}

impl<'a> First for dyn Second where &'a Self: First {
    const CONST: bool = <&Self>::CONST;
}

pub trait Foo {
    const CONST: bool;
}

impl <'a> Foo for () where &'a Self: Foo {
    const CONST: bool = <&Self>::CONST;
}
