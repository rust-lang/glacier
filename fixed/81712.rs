#![feature(generic_associated_types)]

// The cyclic dependency between trait A and B compiles as expected
trait A {
    type BType: B<AType = Self>;
}

trait B {
    type AType: A<BType = Self>;
}

// rustc crashes on the generic cyclic dependency between traits C and D
trait C {
    type DType<T>: D<T, CType = Self>;
}
trait D<T> {
    type CType: C<DType = Self>;
}
