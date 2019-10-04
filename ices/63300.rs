trait Functor<A> {
    type HigherSelf<T>: Functor<T>;
}
