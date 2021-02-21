#![feature(const_generics, const_evaluatable_checked)]

struct True;
struct False;

trait ConstBool {
    type Val;
}
struct TypeBool<const X: bool>;

impl ConstBool for TypeBool<true> {
    type Val = True;
}

impl ConstBool for TypeBool<false> {
    type Val = False;
}

trait Collate<const MASK: u32> {
    type Pass;
    type Fail;

    fn collate(self) -> (Self::Pass, Self::Fail);
}

impl<const MASK: u32> Collate<MASK> for () {
    type Pass = ();
    type Fail = ();

    fn collate(self) -> ((), ()) {
        ((), ())
    }
}

trait CollateStep<X, Prev> {
    type Pass;
    type Fail;
    fn collate_step(x: X, prev: Prev) -> (Self::Pass, Self::Fail);
}

impl<X, P, F> CollateStep<X, (P, F)> for TypeBool<true> {
    type Pass = (X, P);
    type Fail = F;

    fn collate_step(x: X, (p, f): (P, F)) -> ((X, P), F) {
        ((x, p), f)
    }
}

impl<X, P, F> CollateStep<X, (P, F)> for TypeBool<false> {
    type Pass = P;
    type Fail = (X, F);

    fn collate_step(x: X, (p, f): (P, F)) -> (P, (X, F)) {
        (p, (x, f))
    }
}

impl<H, T: Collate<{ MASK >> 1 }>, const MASK: u32> Collate<MASK> for (H, T)
where
    TypeBool<{ 1 == MASK & 1 }>: CollateStep<H, (T::Pass, T::Fail)>,
{
    type Pass =
        <TypeBool<{ 1 == MASK & 1 }> as CollateStep<H, (T::Pass, T::Fail)>>::Pass;
    type Fail =
        <TypeBool<{ 1 == MASK & 1 }> as CollateStep<H, (T::Pass, T::Fail)>>::Fail;

    fn collate(self) -> (Self::Pass, Self::Fail) {
        <TypeBool<{ 1 == MASK & 1 }>
         as CollateStep<H, (T::Pass, T::Fail)>>
         ::collate_step(self.0, self.1.collate())
    }
}

fn collate<X,const MASK:u32>(x:X)->(X::Pass, X::Fail)
where X:Collate<MASK> {
    x.collate()
}

fn main() {
    dbg!(collate::<_,3>((4, ('!',()))));
}
