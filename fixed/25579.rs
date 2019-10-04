[derive(Debug)]
enum Sexpression {
  Id(String),
  Num(f64),
  Cons(Box<(Sexpression, Sexpression)>)
}

fn mutate_each_float<F>(l: &mut Sexpression, f: &mut F)
    where F: FnMut(&mut f64)
{
    let mut l = l;
    loop { match l {
        &mut Sexpression::Id(..) => {},
        &mut Sexpression::Num(ref mut n) => f(n),
        &mut Sexpression::Cons(ref mut expr) => {
            let &mut (ref mut car, ref mut cdr) = &mut **expr;
            mutate_each_float(cdr, f);
            l = car;
        }
    }}
}

fn main() {
}
