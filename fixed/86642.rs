#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]

macro_rules! seq {
    ($( $x:expr ),*) => {
        move |source| {
            $(
                let source = $x(source)?;
            )*
            Ok(source)
        }
    };
}

macro_rules! alt {
    ($first:expr, $( $rest:expr ),*) => {
        move |source| {
            let res = $first(source);
            $(
                let res = res.or($rest(source));
            )*
            res
        }
    };
}

static x: impl Fn(&str) -> Result<&str, ()> = alt!(seq!(), seq!());
