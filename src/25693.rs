pub trait Paramters { type SelfRef; }

struct RP<'a> { _marker: std::marker::PhantomData<&'a ()> }
struct BP;

impl<'a> Paramters for RP<'a> { type SelfRef = &'a X<RP<'a>>; }
impl Paramters for BP { type SelfRef = Box<X<BP>>; }

pub struct Y;
pub enum X<P: Paramters> {
    Nothing,
    SameAgain(P::SelfRef, Y)
}

fn main() {
    // fine
    let bnil: Box<X<BP>> = Box::new(X::Nothing);
    let bx: Box<X<BP>> = Box::new(X::SameAgain(bnil, Y));
    // crashes rustc
    let rnil: X<RP> = X::Nothing;
    let rx: X<RP> = X::SameAgain(&rnil, Y);
}
