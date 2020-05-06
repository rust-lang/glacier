// Note: Centril asked in the issue to
// include all variants in a regression test.

// even more reduced version by Centril
#![feature(type_ascription)]

enum Bug<S> {
    Var = 0: S,
}
