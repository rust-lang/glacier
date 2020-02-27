macro_rules! suite {
    ( $( $fn:ident; )* ) => {
        $(
            const A = "A".$fn();
        )*
    }
}

suite! {
    len;
    is_empty;
}

fn main() {}
