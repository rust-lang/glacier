#![feature(const_generics)]
#![feature(generic_associated_types)]

trait A {
    type B<const N: usize>;
    
    fn foo<const N: usize>(&self) -> Self::B<{N}>;
}