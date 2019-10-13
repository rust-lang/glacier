#![crate_type = "lib"]

#![feature(generic_associated_types)]
trait Iterator {
    type Item<'a>: 'a;
}
