#![recursion_limit = "256000"]

struct Z;
struct S<T>(T);

trait Add<Rhs> {
    type Sum;
}

type SumOf<N, M> = <N as Add<M>>::Sum;

impl<N> Add<N> for Z {
    type Sum = N;
}

impl<N, M> Add<M> for S<N>
where
    N: Add<S<M>>,
{
    type Sum = SumOf<N, S<M>>;
}

type One = S<Z>;
type Two = SumOf<One, One>;
type Three = SumOf<One, Two>;
type Five = SumOf<Two, Three>;
type Ten = SumOf<Five, Five>;
type TwentyFive = SumOf<Five, SumOf<Ten, Ten>>;
type Fifty = SumOf<TwentyFive, TwentyFive>;
type OneHundred = SumOf<Fifty, Fifty>;

type x2 = SumOf<OneHundred, OneHundred>;
type x4 = SumOf<x2, x2>;
type x8 = SumOf<x4, x4>;
type x16 = SumOf<x8, x8>;
type x32 = SumOf<x16, x16>;
type x64 = SumOf<x32, x32>;
type x128 = SumOf<x64, x64>;
//

trait NumericValue {
    const VALUE: usize;
}

impl NumericValue for Z {
    const VALUE: usize = 0;
}

impl<N> NumericValue for S<N>
where
    N: NumericValue,
{
    const VALUE: usize = N::VALUE + 1;
}

const value: usize = <x16 as NumericValue>::VALUE;

fn main() {}
