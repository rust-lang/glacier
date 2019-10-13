fn assert_partialeq<T: PartialEq>(_: &T) {}

fn rquux(_: &()) { }
const RQ: fn(&()) = rquux;
const LRQ: &[fn(&())] = &[];

fn main() {
    // assert_partialeq(&RQ); (RQ isn't PartialEq, and this will say so.)

    match RQ {
        RQ => println!("RQ"),
        _ => {}
    };

    let input: &[fn(&())] = &[][..];
    match input {
        LRQ => println!("LRQ"), // compiles if you comment this out
        &[] => println!("matched inlined pattern"),
        _ => {}
    };
}
