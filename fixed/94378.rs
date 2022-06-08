fn f<'a, 'b, 'd, 'e> (
    x: for<'c> fn(
        fn(&'c fn(&'c ())),
        fn(&'c fn(&'c ())),
        fn(&'c fn(&'c ())),
        fn(&'c fn(&'c ())),
    )
) -> fn(
        fn(&'a fn(&'d ())),
        fn(&'b fn(&'d ())),
        fn(&'a fn(&'e ())),
        fn(&'b fn(&'e ())),
) {
    x
}

fn main() {}
