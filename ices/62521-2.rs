#![crate_type = "lib"]

#![feature(generic_associated_types)]
trait Foo {
  type PublicKey<'a> : From<&'a [u8]>;     
}
